import Identicon from '@polkadot/react-identicon';
import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { WalletModal } from '../wallet-modal';
import { Button } from '../../../../components';

function Wallet() {
  const { account, isAccountReady } = useAccount();

  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);

  const openWalletModal = () => setIsWalletModalOpen(true);
  const closeWalletModal = () => setIsWalletModalOpen(false);

  return isAccountReady ? (
    <>
      <Button variant={account ? 'black' : 'primary'} onClick={openWalletModal}>
        {account && <Identicon value={account.address} size={21} theme="polkadot" />}
        <span>{account ? account.meta.name : 'Connect'}</span>
      </Button>

      {isWalletModalOpen && <WalletModal onClose={closeWalletModal} />}
    </>
  ) : null;
}

export { Wallet };
