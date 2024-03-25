import { Button, Checkbox } from '@gear-js/vara-ui';
import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { ReactComponent as SignlessSVG } from '../../assets/icons/signless.svg';
import { ReactComponent as PowerSVG } from '../../assets/icons/power.svg';
import styles from './enable-signless-session.module.css';
import { CreateSessionModal } from '../create-session-modal';
import { useSignlessTransactions } from '../../context';
import { EnableSessionModal } from '../enable-session-modal';

type Props = {
  type: 'button' | 'switcher';
  shouldIssueVoucher?: boolean;
  message?: string;
  disabled?: boolean;
  onSessionCreate?: (signlessAccountAddress: string) => Promise<void>;
};

function EnableSignlessSession({ type, onSessionCreate, shouldIssueVoucher, disabled, message }: Props) {
  const { account } = useAccount();
  const { isAvailable, pair, session, deletePair, deleteSession } = useSignlessTransactions();
  const [isLoading, setIsLoading] = useState(false);
  const [isCreateSessionModalOpen, setIsCreateSessionModalOpen] = useState(false);
  const [isEnableSessionModalOpen, setIsEnableSessionModalOpen] = useState(false);

  const isSession = !!session;

  const openCreateModal = () => setIsCreateSessionModalOpen(true);
  const closeCreateModal = () => setIsCreateSessionModalOpen(false);

  const openEnableModal = () => setIsEnableSessionModalOpen(true);
  const closeEnableModal = () => setIsEnableSessionModalOpen(false);

  const onDeleteSessionSuccess = () => {
    deletePair();
  };

  const onDeleteSessionFinally = () => {
    setIsLoading(false);
  };

  const handleDeleteSession = async () => {
    if (!session) throw new Error('Signless session not found');
    if (!pair) throw new Error('Signless pair not found');

    setIsLoading(true);
    deleteSession(session.key, pair, { onSuccess: onDeleteSessionSuccess, onFinally: onDeleteSessionFinally });
  };

  const handleSwitcherChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.checked) {
      if (isSession) {
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
          {isSession ? (
            <Button
              icon={PowerSVG}
              text="Disable"
              color="light"
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
              checked={isSession && !!pair}
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
                    <span>Min required: 42 VARA</span>
                  </>
                ) : (
                  message && <span>{message}</span>
                )}
              </span>
            )}
          </div>
        </div>
      )}

      {isCreateSessionModalOpen && (
        <CreateSessionModal
          close={closeCreateModal}
          onSessionCreate={onSessionCreate}
          shouldIssueVoucher={shouldIssueVoucher}
        />
      )}
      {isEnableSessionModalOpen && <EnableSessionModal close={closeEnableModal} />}
    </>
  ) : null;
}

export { EnableSignlessSession };