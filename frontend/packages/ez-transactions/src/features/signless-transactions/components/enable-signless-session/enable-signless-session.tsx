import { useAccount, useAlert } from '@gear-js/react-hooks';
import { Button, Checkbox } from '@gear-js/vara-ui';
import { useState } from 'react';

import PowerSVG from '@ez/assets/icons/power.svg?react';
import SignlessSVG from '@ez/assets/icons/signless.svg?react';

import { useSignlessTransactions } from '../../context';
import { useIsAvailable } from '../../hooks';

import styles from './enable-signless-session.module.css';

type Props = {
  type: 'button' | 'switcher';
  allowedActions: string[];
  shouldIssueVoucher?: boolean;
  message?: string;
  disabled?: boolean;
  onSessionCreate?: (signlessAccountAddress: string) => Promise<`0x${string}`>;
  requiredBalance: number | undefined;
  boundSessionDuration?: number;
};

function EnableSignlessSession(props: Props) {
  const {
    type,
    allowedActions,
    onSessionCreate,
    shouldIssueVoucher,
    disabled,
    message,
    boundSessionDuration,
    requiredBalance = 42,
  } = props;
  const { account } = useAccount();
  const { pair, session, deletePair, deleteSession, isSessionActive, openSessionModal } = useSignlessTransactions();
  const isAvailable = useIsAvailable(requiredBalance, isSessionActive);
  const [isLoading, setIsLoading] = useState(false);
  const alert = useAlert();

  const onError = (error: unknown) => {
    const errorMessage = error instanceof Error ? error.message : String(error);
    alert.error(errorMessage);
    console.error(errorMessage);
  };

  const openCreateModal = () => {
    openSessionModal({
      type: 'create',
      allowedActions,
      onSessionCreate,
      shouldIssueVoucher,
      boundSessionDuration,
    }).catch(onError);
  };

  const openEnableModal = () => {
    openSessionModal({ type: 'enable' }).catch(onError);
  };

  const onDeleteSessionSuccess = () => {
    deletePair();
  };

  const onDeleteSessionFinally = () => {
    setIsLoading(false);
  };

  const handleDeleteSession = () => {
    if (!session) throw new Error('Signless session not found');
    if (!pair) throw new Error('Signless pair not found');

    setIsLoading(true);

    void deleteSession(session.key, pair, {
      onSuccess: onDeleteSessionSuccess,
      onFinally: onDeleteSessionFinally,
    });
  };

  const handleSwitcherChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.checked) {
      if (isSessionActive) {
        openEnableModal();
        return;
      }
      openCreateModal();
    } else {
      handleDeleteSession();
    }
  };

  return account?.decodedAddress ? (
    <>
      {type === 'button' && (
        <>
          {isSessionActive ? (
            <Button
              icon={PowerSVG}
              text="Disable"
              color="grey"
              className={styles.closeButton}
              isLoading={isLoading}
              disabled={!pair}
              onClick={handleDeleteSession}
            />
          ) : (
            <Button
              icon={SignlessSVG}
              color="transparent"
              text="Enable signless transactions"
              disabled={isLoading || !isAvailable || disabled}
              className={styles.enableButton}
              onClick={openCreateModal}
            />
          )}
        </>
      )}

      {type === 'switcher' && (
        <div className={styles.switchContainer}>
          <div className={styles.switcherWrapper}>
            <Checkbox
              label=""
              type="switch"
              disabled={isLoading || !isAvailable || disabled}
              checked={isSessionActive && !!pair}
              onChange={handleSwitcherChange}
            />
          </div>

          <div className={styles.contentWrapper}>
            <div className={styles.headingWrapper}>
              <SignlessSVG />
              <span className={styles.heading}>Enable signless</span>
              {isLoading && <span className={styles.loader} />}
            </div>

            {(!isAvailable || message) && (
              <span className={styles.descr}>
                {!isAvailable ? (
                  <>
                    <span>Not enough balance to enable signless mode.</span>
                    <br />
                    <span>Min required: {requiredBalance} VARA</span>
                  </>
                ) : (
                  message && <span>{message}</span>
                )}
              </span>
            )}
          </div>
        </div>
      )}
    </>
  ) : null;
}

export { EnableSignlessSession };
