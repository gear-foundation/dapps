import { Button, Input, Modal, ModalProps } from '@gear-js/vara-ui';
import { useForm } from 'react-hook-form';

import { useSignlessTransactions } from '../../context';
import styles from '../create-session-modal/create-session-modal.module.css';

type Props = Pick<ModalProps, 'close'>;

const DEFAULT_VALUES = {
  password: '',
};

function EnableSessionModal({ close }: Props) {
  const { register, handleSubmit } = useForm({ defaultValues: DEFAULT_VALUES });
  const { setPassword } = useSignlessTransactions();

  const onSubmit = ({ password }: typeof DEFAULT_VALUES) => {
    setPassword(password);
    close();
  };

  return (
    <Modal heading="Enable Signless Session" close={close}>
      <form onSubmit={handleSubmit(onSubmit)} className={styles.inputs}>
        <Input label="Password" {...register('password')} />
        <Button type="submit" text="Submit" className={styles.button} />
      </form>
    </Modal>
  );
}

export { EnableSessionModal };
