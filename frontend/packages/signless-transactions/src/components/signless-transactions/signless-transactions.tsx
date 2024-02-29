import { Button } from '@gear-js/vara-ui';
import { useAccount, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { useState } from 'react';
import { useCountdown } from '@dapps-frontend/hooks';
import { ReactComponent as SignlessSVG } from '../../assets/icons/signless.svg';
import { ReactComponent as PowerSVG } from '../../assets/icons/power.svg';
import { useSignlessTransactions } from '../../context';
import { getDHMS } from '../../utils';
import { CreateSessionModal } from '../create-session-modal';
import { EnableSessionModal } from '../enable-session-modal';
import styles from './signless-transactions.module.css';
import { SignlessParams } from '../signless-params-list';
import { AccountPair } from '../account-pair';

function SignlessTransactions() {
  const { account } = useAccount();
  const { pair, session, isSessionReady, voucherBalance, storagePair, deletePair, deleteSession } =
    useSignlessTransactions();
  const [modal, setModal] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const openCreateModal = () => setModal('create');
  const openEnableModal = () => setModal('enable');
  const closeModal = () => setModal('');
  const expireTimestamp = +withoutCommas(session?.expires || '0');
  const countdown = useCountdown(expireTimestamp);

  const { getFormattedBalance } = useBalanceFormat();
  const sessionBalance = voucherBalance ? getFormattedBalance(voucherBalance) : undefined;

  const onDeleteSessionSuccess = () => {
    deletePair();
  };

  const onDeleteSessionFinally = () => {
    setIsLoading(false);
  };

  const handleDeleteSession = async () => {
    if (session) {
      setIsLoading(true);
      await deleteSession(session.key, pair, { onSuccess: onDeleteSessionSuccess, onFinally: onDeleteSessionFinally });
    }
  };

  return account && isSessionReady ? (
    <div className={styles.container}>
      {session ? (
        <>
          <div className={styles.buttons}>
            {storagePair ? (
              !pair && (
                <button className={styles.enableButton} onClick={openEnableModal}>
                  <div className={styles.itemIcon}>
                    <SignlessSVG />
                  </div>
                  <span className={styles.itemText}>Unlock signless transactions</span>
                </button>
              )
            ) : (
              <p>Signless account not found in the storage.</p>
            )}
          </div>

          <div className={styles.sessionContainer}>
            <div className={styles.titleWrapper}>
              <SignlessSVG />
              <h3 className={styles.title}>Signless Session is active</h3>
            </div>

            <SignlessParams
              params={[
                {
                  heading: storagePair ? 'Account from the storage:' : 'Randomly generated account:',
                  value: pair ? <AccountPair pair={pair} /> : <span>Inactive</span>,
                },
                {
                  heading: 'Remaining balance:',
                  value: sessionBalance ? `${sessionBalance.value} ${sessionBalance.unit}` : '-',
                },
                {
                  heading: 'Approved Actions:',
                  value: session.allowedActions.join(', '),
                },
                {
                  heading: 'Expires:',
                  value: countdown ? getDHMS(countdown) : '-- : -- : --',
                },
              ]}
            />

            <Button
              icon={PowerSVG}
              text="Log Out"
              color="light"
              className={styles.closeButton}
              isLoading={isLoading}
              disabled={!pair}
              onClick={handleDeleteSession}
            />
          </div>
        </>
      ) : (
        <Button
          icon={SignlessSVG}
          color="transparent"
          text="Enable signless transactions"
          className={styles.enableButton}
          onClick={openCreateModal}
        />
      )}

      {modal === 'enable' && <EnableSessionModal close={closeModal} />}
      {modal === 'create' && <CreateSessionModal close={closeModal} />}
    </div>
  ) : null;
}

export { SignlessTransactions };
