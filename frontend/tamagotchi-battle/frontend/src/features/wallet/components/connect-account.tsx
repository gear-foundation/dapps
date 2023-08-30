import { Button } from '@gear-js/ui';
import { WalletModal } from './wallet-modal';
import { useState } from 'react';

export function ConnectAccount() {
  const [open, setOpen] = useState(false);
  const openModal = () => setOpen(true);
  const closeModal = () => setOpen(false);

  return (
    <>
      <Button text="Connect account" color="primary" onClick={openModal} />
      {open && <WalletModal close={closeModal} />}
    </>
  );
}
