import { Button, Input, Modal } from '@gear-js/ui';
import { isHex } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';
import { useForm } from 'react-hook-form';

import styles from './AddressModal.module.scss';

type Props = {
  heading: string;
  close: () => void;
  onSubmit: (price: HexString) => void;
};

const defaultValues = { address: '' as HexString };

function AddressModal({ heading, close, onSubmit }: Props) {
  const { register, handleSubmit, formState } = useForm({ defaultValues });
  const { errors } = formState;

  return (
    <Modal heading={heading} close={close}>
      <form className={styles.form} onSubmit={handleSubmit(({ address }) => onSubmit(address))}>
        <div>
          <Input
            placeholder="Enter the address"
            {...register('address', { validate: (address) => isHex(address) || 'Address should be hex' })}
          />
          <p className={styles.error}>{errors.address?.message}</p>
        </div>
        <Button type="submit" text="OK" block />
      </form>
    </Modal>
  );
}

export { AddressModal };
