import { Button, Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { useApi } from '@gear-js/react-hooks';
import { isHex } from '@polkadot/util';
import { useAtom } from 'jotai';
import { Modal } from 'components';
import { LOCAL_STORAGE } from 'consts';
import { CONTRACT_ADDRESS_ATOM } from '../../consts';
import styles from './ContractAddressModal.module.scss';

const initialValues = { address: '' };

type Props = {
  onClose: () => void;
};

function ContractAddressModal({ onClose }: Props) {
  const { api } = useApi();
  const [, setAddress] = useAtom(CONTRACT_ADDRESS_ATOM);

  const { getInputProps, onSubmit, setFieldError } = useForm({ initialValues });

  const handleSubmit = onSubmit(async ({ address }) => {
    if (!isHex(address)) return setFieldError('address', 'Address should be hex');

    api.program
      .exists(address)
      .then((isProgramExists) => {
        if (!isProgramExists) throw new Error('Program not found in the storage');

        setAddress(address);
        localStorage.setItem(LOCAL_STORAGE.CONTRACT_ADDRESS, address);
        onClose();
      })
      .catch(({ message }) => setFieldError('address', message));
  });

  return (
    <Modal heading="Contract Address" onClose={onClose}>
      <form className={styles.form} onSubmit={handleSubmit}>
        <Input placeholder="0x01" {...getInputProps('address')} />
        <Button type="submit" text="Submit" block />
      </form>
    </Modal>
  );
}

export { ContractAddressModal };
