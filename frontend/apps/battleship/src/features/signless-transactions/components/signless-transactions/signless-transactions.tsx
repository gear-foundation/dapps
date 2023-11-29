import { Button } from '@gear-js/vara-ui';
import { useAccount, withoutCommas } from '@gear-js/react-hooks';
import { useState } from 'react';

import { useCountdown } from '@dapps-frontend/hooks';

import { useSession } from '../../hooks';
import { getHMS } from '../../utils';
import { CreateSessionModal } from '../create-session-modal';
import styles from './signless-transactions.module.css';

function SignlessTransactions() {
  const { account } = useAccount();

  const { session, isSessionReady } = useSession();

  const expireTimestamp = +withoutCommas(session?.expires || '0');
  const currentTimestamp = Date.now();
  const isSessionActive = currentTimestamp < expireTimestamp;

  const countdown = useCountdown(expireTimestamp);

  const [isModalOpen, setIsModalOpen] = useState(false);
  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  return account && isSessionReady ? (
    isSessionActive ? (
      <div className={styles.active}>
        <p>Signless Session is Active</p>
        <p>Expires: {countdown ? getHMS(countdown) : '-- : -- : --'}</p>
        <p>Approved Actions: {session?.allowedActions.join()}</p>
      </div>
    ) : (
      <>
        <div>
          <Button text="Enable Signless Transactions" size="small" color="dark" onClick={openModal} />
        </div>

        {isModalOpen && <CreateSessionModal close={closeModal} />}
      </>
    )
  ) : null;
}

export { SignlessTransactions };
