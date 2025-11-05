import { useAlert, useApi } from '@gear-js/react-hooks';
import { Button, checkboxStyles } from '@gear-js/ui';
import clsx from 'clsx';
import { useState } from 'react';

import {
  useBalanceOfQuery,
  useCancelSubscriptionMessage,
  useGetSubscriberQuery,
  useRegisterSubscriptionMessage,
} from '@/app/utils';
import { useApproveMessage } from '@/app/utils/sails/messages/use-approve-message';
import { useCurrenciesQuery } from '@/app/utils/sails/queries/use-currencies-query';
import { Period } from '@/app/utils/sails/varatube';
import pic from '@/assets/images/pic.png';
import { Heading, Loader, PurchaseSubscriptionModal } from '@/components';
import { PurchaseSubscriptionApproveModal } from '@/components/modals/purchase-subscription-approve-modal';
import { ENV, periods } from '@/consts';
import { FormValues } from '@/types';

import styles from './Subscription.module.scss';

function Subscription() {
  const { currencies } = useCurrenciesQuery();
  const amount = currencies?.[0][1] ? Number(currencies[0][1]) : null;

  const alert = useAlert();
  const { api } = useApi();
  const { balance, refetch: refetchBalance } = useBalanceOfQuery();
  const [valuesToTransfer, setValuesToTransfer] = useState<FormValues | null>(null);

  const { subscriber, isFetching, refetch } = useGetSubscriberQuery();

  const [isSubscribing, setIsSubscribing] = useState<boolean>(false);

  const { period, start_date, end_date, will_renew } = subscriber || {};

  const startDate = start_date ? new Date(Number(start_date)).toLocaleString() : '';
  const endDate = end_date ? new Date(Number(end_date)).toLocaleString() : '';

  const { registerSubscriptionMessage } = useRegisterSubscriptionMessage();
  const { cancelSubscriptionMessage } = useCancelSubscriptionMessage();
  const { approveMessage } = useApproveMessage();

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isApproveModalOpen, setIsApproveModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  const closeApproveModal = () => setIsApproveModalOpen(false);
  const openApproveModal = () => setIsApproveModalOpen(true);

  const cancelSubscription = async () => {
    await cancelSubscriptionMessage({
      onSuccess: () => {
        void refetch();
        alert.success('Unsubscribed successfully');
      },
      onError: () => {
        alert.error('Gas calculation error');
      },
    });
  };

  const clearValues = () => {
    setValuesToTransfer(null);
  };

  const findSelectedPeriodRate = (targetPeriod: string) =>
    periods.find((item) => item.value === targetPeriod)?.rate || 1;
  const price = period && amount ? String(findSelectedPeriodRate(period) * amount) : null;

  const purchaseSubscription = async () => {
    if (valuesToTransfer) {
      await registerSubscriptionMessage(
        {
          currency_id: ENV.FT_CONTRACT,
          period: valuesToTransfer.period as Period,
          with_renewal: valuesToTransfer.isRenewal,
        },
        {
          onSuccess: () => {
            closeModal();
            clearValues();
            setIsSubscribing(false);
            alert.success('Subscribed successfully');
            void refetch();
            void refetchBalance();
          },
          onError: () => {
            alert.error('Gas calculation error');
            setIsSubscribing(false);
          },
        },
      );
    }
  };

  const saveSubscriptionValues = (values: { isRenewal: boolean; period: string }) => {
    setValuesToTransfer(values);
    openApproveModal();
  };

  const handleApproveStuff = async () => {
    if (!valuesToTransfer || !api) {
      return;
    }

    if (!amount || !balance || Number(balance) < findSelectedPeriodRate(valuesToTransfer?.period) * amount) {
      alert.error(`You don't have enough tokens to subscribe`);
      clearValues();
      closeApproveModal();
      closeModal();
      return;
    }

    setIsSubscribing(true);

    const amountToTransfer = findSelectedPeriodRate(valuesToTransfer.period) * amount;
    if (amountToTransfer) {
      await approveMessage(
        {
          spender: ENV.CONTRACT,
          value: amountToTransfer,
        },
        {
          onSuccess: () => {
            closeApproveModal();
            void purchaseSubscription();
          },
          onError: () => {
            clearValues();
            closeApproveModal();
            closeModal();
            setIsSubscribing(false);
            alert.error('Some error has occured');
          },
        },
      );
    }
  };

  return (
    <>
      {!isFetching && amount ? (
        <>
          <Heading text="My Subscription" />

          <div className={styles.main}>
            {subscriber ? (
              <>
                <div className={styles.subWrapper}>
                  <ul className={styles.list}>
                    <li>
                      Start Date: <span className={styles.value}>{startDate}</span>
                    </li>

                    {endDate && (
                      <li>
                        End Date: <span className={styles.value}>{endDate}</span>
                      </li>
                    )}
                    <li>
                      Period: <span className={styles.value}>{period}</span>
                    </li>

                    {!!price && (
                      <li>
                        Price: <span className={styles.value}>{String(price)}</span>
                      </li>
                    )}

                    <li>
                      Auto-renewal:
                      <input
                        type="checkbox"
                        className={clsx(checkboxStyles.input, checkboxStyles.checkbox)}
                        checked={will_renew}
                        readOnly
                      />
                    </li>
                  </ul>

                  <img src={pic} alt="" />
                </div>

                {will_renew && <Button text="Cancel subscription" color="light" onClick={cancelSubscription} />}
              </>
            ) : (
              <>
                <img src={pic} alt="" className={styles.noSubPic} />

                <div style={{ textAlign: 'center' }}>
                  <p style={{ marginBottom: '4px' }}>You don&apos;t have an active subscription.</p>
                  <p>Please subscribe to get access to app content.</p>
                </div>

                <Button text="Subscribe" disabled={isSubscribing} onClick={openModal} />
              </>
            )}
          </div>
        </>
      ) : (
        <Loader />
      )}

      {isModalOpen && (
        <PurchaseSubscriptionModal
          disabledSubmitButton={isSubscribing}
          close={closeModal}
          onSubmit={saveSubscriptionValues}
        />
      )}
      {isApproveModalOpen && valuesToTransfer && amount && (
        <PurchaseSubscriptionApproveModal
          amount={String(findSelectedPeriodRate(valuesToTransfer.period) * amount)}
          disabledSubmitButton={isSubscribing}
          close={closeApproveModal}
          onSubmit={handleApproveStuff}
        />
      )}
    </>
  );
}

export { Subscription };
