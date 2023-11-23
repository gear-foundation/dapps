import { Button } from '@gear-js/ui';
import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';
import { AccountButton } from '../account-button';
import { WalletModal } from '../wallet-modal';

function Wallet() {
  const { account, isAccountReady } = useAccount();

  const [isModalOpen, setIsModalOpen] = useState(false);
  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  return isAccountReady ? (
    <>
      {account ? (
        <AccountButton address={account.address} name={account.meta.name} onClick={openModal} />
      ) : (
        <Button text="Connect Wallet" color="lightGreen" onClick={openModal} />
      )}

      {isModalOpen && <WalletModal close={closeModal} />}
    </>
  ) : null;
}

export { Wallet };
