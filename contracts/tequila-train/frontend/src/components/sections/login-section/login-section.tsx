import { SelectAccountPopup } from 'components/popups/select-account-popup';
import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';

export const LoginSection = () => {
  const { accounts } = useAccount();
  const [open, setOpen] = useState(false);
  const openModal = () => setOpen(true);
  const closeModal = () => setOpen(false);

  return (
    <>
      <button className="btn btn--primary" onClick={openModal}>
        Connect account
      </button>
      {open && <SelectAccountPopup accounts={accounts} close={closeModal} />}
    </>
  );
};
