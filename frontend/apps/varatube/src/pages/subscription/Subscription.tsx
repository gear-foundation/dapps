import { useAccount, useAlert, useApi, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { Button, checkboxStyles } from '@gear-js/ui';
import { useState } from 'react';
import { Heading, Loader, PurchaseSubscriptionModal } from 'components';
import { useSubscriptionsMessage } from 'hooks';
import { useHandleCalculateGas, useCheckBalance } from '@dapps-frontend/hooks';
import varatubeMeta from 'assets/state/varatube_meta.txt';
import pic from 'assets/images/pic.png';
import clsx from 'clsx';
import { ADDRESS, periods } from 'consts';
import styles from './Subscription.module.scss';
import { PurchaseSubscriptionApproveModal } from 'components/modals/purchase-subscription-approve-modal';
import { InitialValues } from 'types';
import { useFTBalance, useFTMessage, useProgramState } from 'hooks/api';
import { useProgramMetadata } from 'hooks/metadata';

function Subscription() {
  const amount = 10000;
  const { account } = useAccount();
  const alert = useAlert();
  const { api } = useApi();
  const tokens = useFTBalance();
  const { decodedAddress } = account || {};
  const [valuesToTransfer, setValuesToTransfer] = useState<InitialValues | null>(null);
  const { subscriptionsState, isSubscriptionsStateRead, updateState } = useProgramState();
  const varatubeMetadata = useProgramMetadata(varatubeMeta);
  const calculateGas = useHandleCalculateGas(ADDRESS.CONTRACT, varatubeMetadata);
  const { checkBalance } = useCheckBalance(ADDRESS.CONTRACT);
  const subscription = subscriptionsState && decodedAddress ? subscriptionsState[decodedAddress] : undefined;
  const [isSubscribing, setIsSubscribing] = useState<boolean>(false);

  const { period, price, willRenew, subscriptionStart, subscriptionEnd } = subscription || {};
  const [startDateTimestamp] = subscriptionStart || [];
  const [endDateTimestamp] = subscriptionEnd || [];

  const startDate = startDateTimestamp ? new Date(+withoutCommas(startDateTimestamp)).toLocaleString() : '';
  const endDate = endDateTimestamp ? new Date(+withoutCommas(endDateTimestamp)).toLocaleString() : '';

  const sendMessage = useSubscriptionsMessage();
  const sendFTMessage = useFTMessage();

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [isApproveModalOpen, setIsApproveModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  const closeApproveModal = () => setIsApproveModalOpen(false);
  const openApproveModal = () => setIsApproveModalOpen(true);

  const cancelSubscription = () => {
    const payload = { CancelSubscription: null };

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string);
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);

        checkBalance(gasLimit, () => {
          sendMessage({
            payload,
            gasLimit,
            onSuccess: () => {
              updateState();
              alert.success('Unsubscribed successfully');
            },
          });
        });
      })
      .catch((error) => {
        console.log(error);
        alert.error('Gas calculation error');
      });
  };

  const clearValues = () => {
    setValuesToTransfer(null);
  };

  const findSelectedPeriodRate = (period: string) => periods.find((item) => item.value === period)?.rate || 1;

  const purchaseSubscription = () => {
    if (valuesToTransfer) {
      const payload = {
        RegisterSubscription: {
          currency_id: ADDRESS.FT_CONTRACT,
          period: { [valuesToTransfer.period]: null },
          with_renewal: valuesToTransfer.isRenewal,
        },
      };

      calculateGas(payload)
        .then((res) => res.toHuman())
        .then(({ min_limit }) => {
          const minLimit = withoutCommas(min_limit as string);
          const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);

          checkBalance(gasLimit, () => {
            sendMessage({
              payload,
              gasLimit,
              onSuccess: () => {
                closeModal();
                clearValues();
                updateState();
                setIsSubscribing(false);
                alert.success('Subscribed successfully');
              },
            });
          });
        })
        .catch((error) => {
          console.log(error);
          alert.error('Gas calculation error');
          setIsSubscribing(false);
        });
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

    if (!tokens || Number(withoutCommas(tokens)) < findSelectedPeriodRate(valuesToTransfer?.period) * amount) {
      alert.error(`You don't have enough tokens to subscribe`);
      clearValues();
      closeApproveModal();
      closeModal();
      return;
    }

    setIsSubscribing(true);

    checkBalance(api?.blockGasLimit.toNumber(), () => {
      const amountToTransfer = findSelectedPeriodRate(valuesToTransfer.period) * amount;
      if (amountToTransfer)
        sendFTMessage({
          payload: {
            Approve: {
              to: ADDRESS.CONTRACT,
              amount: String(findSelectedPeriodRate(valuesToTransfer.period) * amount),
            },
          },
          onSuccess: () => {
            closeApproveModal();
            purchaseSubscription();
          },
          onError: () => {
            clearValues();
            closeApproveModal();
            closeModal();
            setIsSubscribing(false);
            alert.error('Some error has occured');
          },
        });
    });
  };

  return (
    <>
      {isSubscriptionsStateRead ? (
        <>
          <Heading text="My Subscription" />

          <div className={styles.main}>
            {subscription ? (
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

                    {price && (
                      <li>
                        Price: <span className={styles.value}>{price}</span>
                      </li>
                    )}

                    <li>
                      Auto-renewal:
                      <input
                        type="checkbox"
                        className={clsx(checkboxStyles.input, checkboxStyles.checkbox)}
                        checked={willRenew}
                        readOnly
                      />
                    </li>
                  </ul>

                  <img src={pic} alt="" />
                </div>

                <Button text="Cancel subscription" color="light" onClick={cancelSubscription} />
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
      {isApproveModalOpen && valuesToTransfer && (
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
