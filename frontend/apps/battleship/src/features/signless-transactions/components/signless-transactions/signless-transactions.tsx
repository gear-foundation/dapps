import { Button } from '@gear-js/vara-ui';
import { useAccount, withoutCommas } from '@gear-js/react-hooks';
import { useState } from 'react';

import { useCountdown } from '@dapps-frontend/hooks';

import { useSession } from '../../hooks';
import { getHMS } from '../../utils';
import { CreateSessionModal } from '../create-session-modal';
import styles from './signless-transactions.module.css';
import { useSignlessTransactions } from '../../context';
import { EnableSessionModal } from '../enable-session-modal';

function SignlessTransactions() {
  const { account } = useAccount();
  const { password } = useSignlessTransactions();

  const { session, isSessionReady } = useSession();

  const expireTimestamp = +withoutCommas(session?.expires || '0');
  const currentTimestamp = Date.now();
  const isSessionActive = currentTimestamp < expireTimestamp;

  const countdown = useCountdown(expireTimestamp);

  const [modal, setModal] = useState('');
  const openCreateModal = () => setModal('create');
  const openEnableModal = () => setModal('enable');
  const closeModal = () => setModal('');

  return account && isSessionReady ? (
    <>
      {isSessionActive ? (
        <>
          {!password && (
            <Button text="Unlock Signless Transactions" size="small" color="dark" onClick={openEnableModal} />
          )}

          <div className={styles.active}>
            <p>Signless Session is Active</p>
            <p>Expires: {countdown ? getHMS(countdown) : '-- : -- : --'}</p>
            <p>Approved Actions: {session?.allowedActions.join()}</p>
          </div>
        </>
      ) : (
        <div>
          <Button text="Enable Signless Transactions" size="small" color="dark" onClick={openCreateModal} />
        </div>
      )}

      {modal === 'enable' && <EnableSessionModal close={closeModal} />}
      {modal === 'create' && <CreateSessionModal close={closeModal} />}
    </>
  ) : null;
}

export { SignlessTransactions };
