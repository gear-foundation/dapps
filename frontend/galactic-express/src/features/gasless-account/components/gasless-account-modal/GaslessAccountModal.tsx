import { Button, Modal } from '@gear-js/ui';
import { useState } from 'react';
import { useGaslessAccount } from '../../Context';
import { LoginForm } from '../login-form';
import styles from './GaslessAccountModal.module.scss';
import { RegistrationForm } from '../registration-form';

type Props = {
  onClose: () => void;
};

function GaslessAccountModal({ onClose }: Props) {
  const { isLoggedIn, account, logout } = useGaslessAccount();

  const [form, setForm] = useState('');

  const handleLogoutButtonClick = () => {
    logout();
    onClose();
  };

  const closeForm = () => setForm('');
  const openLoginForm = () => setForm('login');
  const openRegistrationForm = () => setForm('registration');

  const getForm = () => {
    if (form === 'login') return <LoginForm onSubmit={onClose} />;
    if (form === 'registration') return <RegistrationForm onSubmit={openLoginForm} />;
  };

  return (
    <Modal heading="Gasless Account" close={onClose}>
      {isLoggedIn ? (
        <>
          <div className={styles.keys}>
            <p>
              <span>Public key:</span>
              <span>{account.publicKey}</span>
            </p>

            <p>
              <span>Private key:</span>
              <span>{account.privateKey}</span>
            </p>
          </div>

          <Button text="Logout" color="light" onClick={handleLogoutButtonClick} block />
        </>
      ) : (
        getForm() || (
          <div className={styles.buttons}>
            <Button text="Login" color="lightGreen" size="small" onClick={openLoginForm} />
            <Button text="Registration" color="lightGreen" size="small" onClick={openRegistrationForm} />
          </div>
        )
      )}

      {form && <Button text="Go Back" color="light" onClick={closeForm} className={styles.backButton} block />}
    </Modal>
  );
}

export { GaslessAccountModal };
