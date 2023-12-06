import { Button } from '@gear-js/vara-ui';
import { useAccount, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { useState } from 'react';

import { useCountdown } from '@dapps-frontend/hooks';

import { useSignlessTransactions } from '../../context';
import { useCreateSession } from '../../hooks';
import { getHMS } from '../../utils';
import { CreateSessionModal } from '../create-session-modal';
import { EnableSessionModal } from '../enable-session-modal';
import styles from './signless-transactions.module.css';

function SignlessTransactions() {
  const { account } = useAccount();
  const { pair, session, isSessionReady, voucherBalance } = useSignlessTransactions();
  const { deleteSession } = useCreateSession();

  const [modal, setModal] = useState('');
  const openCreateModal = () => setModal('create');
  const openEnableModal = () => setModal('enable');
  const closeModal = () => setModal('');

  const expireTimestamp = +withoutCommas(session?.expires || '0');
  const countdown = useCountdown(expireTimestamp);

  const { getFormattedBalance } = useBalanceFormat();
  const sessionBalance = voucherBalance ? getFormattedBalance(voucherBalance) : undefined;

  return account && isSessionReady ? (
    <div>
      {session ? (
        <>
          <div className={styles.buttons}>
            {!pair && (
              <Button text="Unlock Signless Transactions" size="small" color="dark" onClick={openEnableModal} />
            )}

            <Button
              text="Close Signless Session"
              size="small"
              color="transparent"
              className={styles.closeButton}
              onClick={deleteSession}
            />
          </div>

          <div className={styles.session}>
            <p>Signless Session is Active</p>
            <p>Expires: {countdown ? getHMS(countdown) : '-- : -- : --'}</p>
            <p>Approved Actions: {session.allowedActions.join(', ')}</p>

            {sessionBalance && (
              <p>
                Remaining Balance: {sessionBalance.value} {sessionBalance.unit}
              </p>
            )}
          </div>
        </>
      ) : (
        <Button text="Enable Signless Transactions" size="small" color="dark" onClick={openCreateModal} />
      )}

      {modal === 'enable' && <EnableSessionModal close={closeModal} />}
      {modal === 'create' && <CreateSessionModal close={closeModal} />}
    </div>
  ) : null;
}

export { SignlessTransactions };
