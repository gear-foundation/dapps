import { useForm } from '@mantine/form';
import { useApi } from '@gear-js/react-hooks';
import { isHex } from '@polkadot/util';
import { useAtom } from 'jotai';
import { Modal } from 'components';
import { CONTRACT_ADDRESS_ATOM } from '../../consts';
import styles from './ContractAddressModal.module.scss';

const initialValues = { address: '' };

type Props = {
  onClose: () => void;
};

function ContractAddressModal({ onClose }: Props) {
  const { api } = useApi();
  const [, setAddress] = useAtom(CONTRACT_ADDRESS_ATOM);

  const { getInputProps, onSubmit, setFieldError, errors } = useForm({ initialValues });
  const error = errors.address;

  const handleSubmit = onSubmit(async ({ address }) => {
    if (!isHex(address)) return setFieldError('address', 'Address should be hex');

    api.program
      .exists(address)
      .then((isProgramExists) => {
        if (!isProgramExists) throw new Error('Program not found in the storage');

        setAddress(address);
        onClose();
      })
      .catch(({ message }) => setFieldError('address', message));
  });

  return (
    <Modal heading="Contract Address" onClose={onClose}>
      <form className={styles.form} onSubmit={handleSubmit}>
        <div>
          {/* eslint-disable-next-line react/jsx-props-no-spreading */}
          <input placeholder="0x01" {...getInputProps('address')} className={styles.input} />
          {error && <p className={styles.error}>{error}</p>}
        </div>

        <button type="submit" className={styles.button}>
          Submit
        </button>
      </form>
    </Modal>
  );
}

export { ContractAddressModal };
