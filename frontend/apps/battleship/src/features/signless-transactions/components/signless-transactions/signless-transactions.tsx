import { Button } from '@gear-js/vara-ui';
import { useState } from 'react';

import { useSession } from '../../hooks';
import { CreateSessionModal } from '../create-session-modal';
import styles from './signless-transactions.module.css';
import { useAccount } from '@gear-js/react-hooks';

function SignlessTransactions() {
  const { account } = useAccount();

  const { session, isSessionReady } = useSession();

  const [isModalOpen, setIsModalOpen] = useState(false);
  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  console.log('session: ', session);

  return account ? (
    <>
      <div>
        <Button
          text="Enable Signless Transactions"
          size="small"
          color="dark"
          isLoading={!isSessionReady}
          onClick={openModal}
        />
      </div>

      {isModalOpen && <CreateSessionModal close={closeModal} />}
    </>
  ) : null;
}

export { SignlessTransactions };
