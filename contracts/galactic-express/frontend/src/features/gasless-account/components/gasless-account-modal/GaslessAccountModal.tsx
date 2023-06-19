import { useAlert } from '@gear-js/react-hooks';
import { Button, Input, Modal } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { ADDRESS } from 'consts';
import { useGaslessAccount } from '../../Context';
import styles from './GaslessAccountModal.module.scss';

const initialValues = { login: '', password: '' };

type Props = {
  onClose: () => void;
};

function GaslessAccountModal({ onClose }: Props) {
  const { getInputProps, onSubmit } = useForm({ initialValues });
  const alert = useAlert();
  const { isLoggedIn, account, setAccount, logout } = useGaslessAccount();

  const handleSubmit = onSubmit(({ login, password }) => {
    fetch(`${ADDRESS.GASLESS_API}/get_keys`, {
      method: 'POST',
      body: JSON.stringify({ nickname: login, password }),
      headers: { 'Content-Type': 'application/json' },
    })
      .then((response) => {
        if (!response.ok) throw new Error(response.statusText);

        return response.json();
      })
      .then(({ publicKey, privateKey }) => {
        setAccount({ publicKey, privateKey });
        onClose();
      })
      .catch((error: Error) => {
        alert.error(error.message);
        // eslint-disable-next-line no-console
        console.error(error);
      });
  });

  const handleLogoutButtonClick = () => {
    logout();
    onClose();
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

          <Button text="Logout" onClick={handleLogoutButtonClick} block />
        </>
      ) : (
        <form onSubmit={handleSubmit} className={styles.form}>
          <div className={styles.inputs}>
            <Input label="Login:" direction="y" {...getInputProps('login')} />

            <Input type="password" label="Password:" direction="y" {...getInputProps('password')} />
          </div>

          <Button type="submit" text="Login" block />
        </form>
      )}
    </Modal>
  );
}

export { GaslessAccountModal };
