import { generateVoucherId } from '@gear-js/api';
import { Button } from '@gear-js/vara-ui';
import { useAccount, useBalance, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { useState } from 'react';

import { useCountdown } from '@dapps-frontend/hooks';

import { ADDRESS } from '@/app/consts';

import { useSignlessTransactions } from '../../context';
import { getHMS } from '../../utils';
import { CreateSessionModal } from '../create-session-modal';
import { EnableSessionModal } from '../enable-session-modal';
import styles from './signless-transactions.module.css';

function SignlessTransactions() {
  const { account } = useAccount();
  const { pair, session, isSessionReady } = useSignlessTransactions();

  const [modal, setModal] = useState('');
  const openCreateModal = () => setModal('create');
  const openEnableModal = () => setModal('enable');
  const closeModal = () => setModal('');

  const expireTimestamp = +withoutCommas(session?.expires || '0');
  const countdown = useCountdown(expireTimestamp);

  const voucherId = session ? generateVoucherId(session.key, ADDRESS.GAME) : undefined;
  const { balance } = useBalance(voucherId);
  const { getFormattedBalance } = useBalanceFormat();
  const sessionBalance = balance ? getFormattedBalance(balance) : undefined;

  return account && isSessionReady ? (
    <>
      {session ? (
        <>
          {!pair && <Button text="Unlock Signless Transactions" size="small" color="dark" onClick={openEnableModal} />}

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
