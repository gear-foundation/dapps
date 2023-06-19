import { useAlert } from '@gear-js/react-hooks';
import { Button, Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { ADDRESS } from 'consts';
import { useNavigate } from 'react-router-dom';
import { useGaslessAccount } from '../../Context';
import styles from './Login.module.scss';

const initialValues = { login: '', password: '' };

function Login() {
  const { getInputProps, onSubmit } = useForm({ initialValues });
  const alert = useAlert();
  const navigate = useNavigate();
  const { setAccount } = useGaslessAccount();

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
        navigate('/');
      })
      .catch((error: Error) => {
        alert.error(error.message);
        // eslint-disable-next-line no-console
        console.error(error);
      });
  });

  return (
    <form onSubmit={handleSubmit} className={styles.form}>
      <div className={styles.inputs}>
        <Input label="Login:" direction="y" {...getInputProps('login')} />

        <Input type="password" label="Password:" direction="y" {...getInputProps('password')} />
      </div>

      <Button type="submit" text="Login" />
    </form>
  );
}

export { Login };
