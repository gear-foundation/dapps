import { useForm } from '@mantine/form';
import { Button, Input } from '@gear-js/ui';
import { mnemonicGenerate } from '@polkadot/util-crypto';
import { Keyring } from '@polkadot/api';
import { useAlert } from '@gear-js/react-hooks';
import { ADDRESS } from 'consts';
import styles from './RegistrationForm.module.scss';

const initialValues = { login: '', password: '', confirmPassword: '' };

type Props = {
  onSubmit: () => void;
};

function RegistrationForm({ onSubmit }: Props) {
  const form = useForm({ initialValues });
  const { getInputProps, setFieldError } = form;

  const alert = useAlert();

  const handleSubmit = form.onSubmit(({ login, password, confirmPassword }) => {
    if (password !== confirmPassword) return setFieldError('confirmPassword', 'Password is not the same');
    if (password.length < 6) return setFieldError('password', 'Minimum length is 6 symbols');
    if (login.length < 4) return setFieldError('login', 'Minimum length is 4 symbols');

    const keyring = new Keyring({ type: 'sr25519' });
    const privateKey = mnemonicGenerate(12);
    const pair = keyring.addFromMnemonic(privateKey);
    const publicKey = pair.address;

    fetch(`${ADDRESS.GASLESS_API}/register`, {
      method: 'POST',
      body: JSON.stringify({
        publicKey,
        privateKey,
        nickname: login,
        password,
      }),
      headers: {
        'Content-Type': 'application/json',
      },
    })
      .then((response) => {
        if (!response.ok) throw new Error(response.statusText);

        alert.success('Registered');
        onSubmit();
      })
      .catch((error: Error) => {
        alert.error(error.message);
        // eslint-disable-next-line no-console
        console.error(error);
      });
  });

  return (
    <form className={styles.form} onSubmit={handleSubmit}>
      <div className={styles.inputs}>
        <Input label="Login:" direction="y" {...getInputProps('login')} />
        <Input type="password" label="Password:" direction="y" {...getInputProps('password')} />
        <Input type="password" label="Confirm Password:" direction="y" {...getInputProps('confirmPassword')} />
      </div>

      <Button type="submit" text="Register" block />
    </form>
  );
}

export { RegistrationForm };
