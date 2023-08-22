import { Button, Checkbox, Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { isHex } from '@polkadot/util';
import clsx from 'clsx';
import { useLotteryMessage } from 'hooks';
import { useEffect, useState } from 'react';
import styles from './Form.module.scss';

const initialValues = { duration: '', participationCost: '', ftActorId: '' };

const getValidation = (isFungibleToken: boolean) => ({
  duration: (value: string) => (!value ? 'Duration is required' : null),
  participationCost: (value: string) => (!value ? 'Participation cost is required' : null),
  ftActorId: (value: string) => (isFungibleToken && !isHex(value) ? 'Address should be hex' : null),
});

const S_MULTIPLIER = 60;
const MS_MULTIPLIER = 1000;

function Form() {
  const [isFungibleToken, setIsFungibleToken] = useState(false);
  const toggleToken = () => setIsFungibleToken((prevValue) => !prevValue);

  const form = useForm({ initialValues, validate: getValidation(isFungibleToken) });
  const { getInputProps, onSubmit, reset, setFieldValue } = form;

  const sendMessage = useLotteryMessage();

  const checkboxClassName = clsx(styles.input, styles.checkbox);

  const resetForm = () => {
    reset();
    setIsFungibleToken(false);
  };

  const handleSubmit = (data: typeof initialValues) => {
    const duration = +data.duration * S_MULTIPLIER * MS_MULTIPLIER;
    const ftActorId = data.ftActorId || null;
    const payload = { Start: { ...data, duration, ftActorId } };

    sendMessage(payload, { onSuccess: resetForm });
  };

  useEffect(() => {
    if (!isFungibleToken) setFieldValue('ftActorId', '');
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isFungibleToken]);

  return (
    <form onSubmit={onSubmit(handleSubmit)}>
      <div className={styles.inputs}>
        <Input
          gap="1/2"
          type="number"
          className={styles.input}
          label="Game duration (minutes)"
          {...getInputProps('duration')}
        />
        <Input
          gap="1/2"
          type="number"
          className={styles.input}
          label="Cost of participation"
          {...getInputProps('participationCost')}
        />
        <div className={styles.token}>
          <div className={checkboxClassName}>
            <Checkbox label="Fungible token" checked={isFungibleToken} onChange={toggleToken} />
          </div>

          {isFungibleToken && (
            <Input gap="1/2" className={styles.input} label="Address" {...getInputProps('ftActorId')} />
          )}
        </div>
      </div>
      <Button type="submit" text="Submit and start" />
    </form>
  );
}

export { Form };
