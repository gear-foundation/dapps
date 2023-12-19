import { Button } from '@gear-js/vara-ui';
import { useAccount, useAlert, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { useState } from 'react';
import { ReactComponent as FileCopyIcon } from '../../assets/icons/file-copy-fill.svg';
import { useCountdown } from '@dapps-frontend/hooks';
import { ReactComponent as SignlessIcon } from '../../assets/icons/signless.svg';
import { ReactComponent as PowerIcon } from '../../assets/icons/power.svg';
import { useSignlessTransactions } from '../../context';
import { copyToClipboard, getHMS, getVaraAddress, shortenString } from '../../utils';
import { CreateSessionModal } from '../create-session-modal';
import { EnableSessionModal } from '../enable-session-modal';
import styles from './signless-transactions.module.css';

function SignlessTransactions() {
  const { account } = useAccount();
  const { pair, session, isSessionReady, voucherBalance, storagePair, deleteSession } = useSignlessTransactions();

  const [modal, setModal] = useState('');
  const openCreateModal = () => setModal('create');
  const openEnableModal = () => setModal('enable');
  const closeModal = () => setModal('');
  const alert = useAlert();
  const expireTimestamp = +withoutCommas(session?.expires || '0');
  const countdown = useCountdown(expireTimestamp);

  const { getFormattedBalance } = useBalanceFormat();
  const sessionBalance = voucherBalance ? getFormattedBalance(voucherBalance) : undefined;

  return account && isSessionReady ? (
    <div className={styles.container}>
      {session ? (
        <>
          <div className={styles.buttons}>
            {storagePair ? (
              !pair && (
                <button className={styles['enable-button']} onClick={openEnableModal}>
                  <div className={styles['item-icon']}>
                    <SignlessIcon />
                  </div>
                  <span className={styles['item-text']}>Unlock signless transactions</span>
                </button>
              )
            ) : (
              <p>Signless account not found in the storage.</p>
            )}
          </div>

          <div className={styles['session-container']}>
            <div className={styles['title-wrapper']}>
              <SignlessIcon />
              <h3 className={styles.title}>Signless Session is active</h3>
            </div>
            <ul className={styles.session}>
              <li className={styles['session-item']}>
                <p className={styles.heading}>
                  {storagePair ? 'Account from the storage:' : 'Randomly generated account:'}
                </p>
                <div className={styles.separator} />
                {pair && (
                  <div className={styles.account}>
                    <span className={styles.value}>{shortenString(getVaraAddress(pair.address), 4)}</span>
                    <FileCopyIcon
                      onClick={() => copyToClipboard({ value: getVaraAddress(pair.address), alert })}
                      className={styles.copy}
                    />
                  </div>
                )}
              </li>
              {sessionBalance && (
                <li className={styles['session-item']}>
                  <p className={styles.heading}>Remaining balance:</p>
                  <div className={styles.separator} />
                  <p className={styles.value}>
                    {sessionBalance.value} {sessionBalance.unit}
                  </p>
                </li>
              )}
              <li className={styles['session-item']}>
                <p className={styles.heading}>Approved Actions: </p>
                <div className={styles.separator} />
                <p className={styles.value}>{session.allowedActions.join(', ')}</p>
              </li>
              <li className={styles['session-item']}>
                <p className={styles.heading}>Expires: </p>
                <div className={styles.separator} />
                <p className={styles.value}>{countdown ? getHMS(countdown) : '-- : -- : --'}</p>
              </li>
            </ul>
            <Button
              icon={PowerIcon}
              text="Log Out"
              color="light"
              className={styles.closeButton}
              onClick={deleteSession}
            />
          </div>
        </>
      ) : (
        <button className={styles['enable-button']} onClick={openCreateModal}>
          <div className={styles['item-icon']}>
            <SignlessIcon />
          </div>
          <span className={styles['item-text']}>Enable signless transactions</span>
        </button>
      )}

      {modal === 'enable' && <EnableSessionModal close={closeModal} />}
      {modal === 'create' && <CreateSessionModal close={closeModal} />}
    </div>
  ) : null;
}

export { SignlessTransactions };
