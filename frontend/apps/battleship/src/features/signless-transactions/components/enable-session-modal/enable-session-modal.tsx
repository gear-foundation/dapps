import { Button, Input, Modal, ModalProps } from '@gear-js/vara-ui';
import { useState } from 'react';
import { useForm } from 'react-hook-form';

import { useSignlessTransactions } from '../../context';
import styles from '../create-session-modal/create-session-modal.module.css';

type Props = Pick<ModalProps, 'close'>;

const DEFAULT_VALUES = {
  password: '',
};

function EnableSessionModal({ close }: Props) {
  const { register, handleSubmit, setError, formState } = useForm({ defaultValues: DEFAULT_VALUES });
  const { errors } = formState;

  const { unlockPair } = useSignlessTransactions();
  const [isLoading, setIsLoading] = useState(false);

  const onSubmit = ({ password }: typeof DEFAULT_VALUES) => {
    setIsLoading(true);

    try {
      unlockPair(password);
      close();
    } catch (error) {
      const message = String(error);

      setError('password', { message });
      setIsLoading(false);
    }
  };

  return (
    <Modal heading="Enable Signless Session" close={close}>
      <form onSubmit={handleSubmit(onSubmit)} className={styles.form}>
        <Input
          type="password"
          label="Password"
          error={errors.password?.message}
          {...register('password', { required: 'Field is required' })}
        />

        <Button type="submit" text="Submit" className={styles.button} isLoading={isLoading} />
      </form>
    </Modal>
  );
}

export { EnableSessionModal };
