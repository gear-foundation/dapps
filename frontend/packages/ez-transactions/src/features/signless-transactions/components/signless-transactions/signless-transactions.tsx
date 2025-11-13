import { decodeAddress } from '@gear-js/api';
import { useAccount, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { useState } from 'react';

import { useCountdown } from '@dapps-frontend/hooks';

import PowerSVG from '@ez/assets/icons/power.svg?react';
import SignlessSVG from '@ez/assets/icons/signless.svg?react';

import { useSignlessTransactions } from '../../context';
import { getDHMS } from '../../utils';
import { AccountPair } from '../account-pair';
import { EnableSignlessSession } from '../enable-signless-session';
import { SignlessParams } from '../signless-params-list';

import styles from './signless-transactions.module.css';

type Props = {
  allowedActions: string[];
  onSessionCreate?: (signlessAccountAddress: string) => Promise<`0x${string}`>;
  shouldIssueVoucher?: boolean;
  disabled?: boolean;
  requiredBalance?: number;
  boundSessionDuration?: number;
};

function SignlessTransactions({
  allowedActions,
  onSessionCreate,
  shouldIssueVoucher,
  disabled,
  requiredBalance,
  boundSessionDuration,
}: Props) {
  const { account } = useAccount();
  const { pair, session, isSessionReady, voucherBalance, storagePair, deletePair, deleteSession, openSessionModal } =
    useSignlessTransactions();
  const [isLoading, setIsLoading] = useState(false);
  const openCreateModal = () => {
    void openSessionModal({
      type: 'create',
      allowedActions,
      onSessionCreate,
      shouldIssueVoucher,
      boundSessionDuration,
    });
  };
  const openEnableModal = () => {
    void openSessionModal({ type: 'enable' });
  };
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

  const handleProlongExpiredSession = () => {
    if (pair) {
      openCreateModal();
    }
  };

  const handleRevokeVoucherFromStoragePair = () => {
    if (!pair) throw new Error('Signless pair not found');

    const decodedAddress = decodeAddress(pair.address);

    setIsLoading(true);

    void deleteSession(decodedAddress, pair, {
      onSuccess: onDeleteSessionSuccess,
      onFinally: onDeleteSessionFinally,
    });
  };

  return account && isSessionReady ? (
    <div className={styles.container}>
      {session && (
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

            <EnableSignlessSession
              type="button"
              allowedActions={allowedActions}
              onSessionCreate={onSessionCreate}
              shouldIssueVoucher={shouldIssueVoucher}
              requiredBalance={requiredBalance}
              boundSessionDuration={boundSessionDuration}
            />
          </div>
        </>
      )}

      {!session && storagePair && (
        <>
          <div className={clsx(styles.titleWrapper, styles.expiredTitleWrapper)}>
            <h3 className={styles.title}>Your Signless Session is expired</h3>
          </div>

          {pair && (
            <div className={styles.expiredButtons}>
              <Button
                icon={SignlessSVG}
                text="Prolong session"
                isLoading={isLoading}
                size="small"
                onClick={handleProlongExpiredSession}
              />

              <Button
                icon={PowerSVG}
                text="Disable session"
                color="grey"
                className={styles.closeButton}
                isLoading={isLoading}
                size="small"
                onClick={handleRevokeVoucherFromStoragePair}
              />
            </div>
          )}
        </>
      )}

      {!session && (
        <EnableSignlessSession
          type="button"
          allowedActions={allowedActions}
          onSessionCreate={onSessionCreate}
          shouldIssueVoucher={shouldIssueVoucher}
          disabled={disabled}
          requiredBalance={requiredBalance}
          boundSessionDuration={boundSessionDuration}
        />
      )}
    </div>
  ) : null;
}

export { SignlessTransactions };
