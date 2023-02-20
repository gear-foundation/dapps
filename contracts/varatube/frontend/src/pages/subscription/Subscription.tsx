import { useAccount } from '@gear-js/react-hooks';
import { Button, Checkbox, checkboxStyles } from '@gear-js/ui';
import { useState } from 'react';
import { Heading, Loader, PurchaseSubscriptionModal } from 'components';
import { useSubscriptions, useSubscriptionsMessage } from 'hooks';
import pic from 'assets/images/pic.png';
import clsx from 'clsx';
import { ADDRESS } from 'consts';
import styles from './Subscription.module.scss';

function Subscription() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { subscriptionsState, isSubscriptionsStateRead } = useSubscriptions();
  const subscription = subscriptionsState && decodedAddress ? subscriptionsState[decodedAddress] : undefined;

  const { startDate: startDateTimestamp, period, endDate: endDateTimestamp, price, willRenew } = subscription || {};

  const startDate = startDateTimestamp ? new Date(startDateTimestamp).toLocaleString() : '';
  const endDate = endDateTimestamp ? new Date(endDateTimestamp).toLocaleString() : '';

  const sendMessage = useSubscriptionsMessage();

  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  const cancelSubscription = () => sendMessage({ CancelSubscription: null });

  const purchaseSubscription = (values: { isRenewal: boolean; period: string }) =>
    sendMessage(
      {
        RegisterSubscription: {
          payment_method: ADDRESS.FT_CONTRACT,
          period: { [values.period]: null },
          with_renewal: values.isRenewal,
        },
      },
      { onSuccess: closeModal },
    );

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

                    <li>
                      End Date: <span className={styles.value}>{endDate}</span>
                    </li>

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

                <Button text="Subscribe" onClick={openModal} />
              </>
            )}
          </div>
        </>
      ) : (
        <Loader />
      )}

      {isModalOpen && <PurchaseSubscriptionModal close={closeModal} onSubmit={purchaseSubscription} />}
    </>
  );
}

export { Subscription };
