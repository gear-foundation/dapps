import { useForm } from '@mantine/form';
import { useApi } from '@gear-js/react-hooks';
import { Modal } from 'components';
import { isProgramIdValid } from 'utils';
import { useContractAddress } from 'features/contract-address/hooks';
import styles from './ContractAddressModal.module.scss';

const initialValues = { address: '' };

type Props = {
  onClose: () => void;
};

function ContractAddressModal({ onClose }: Props) {
  const { api } = useApi();
  const { setContractAddress } = useContractAddress();

  const { getInputProps, onSubmit, setFieldError, errors } = useForm({ initialValues });
  const error = errors.address;

  const handleSubmit = onSubmit(async ({ address }) => {
    if (!isProgramIdValid(address)) return setFieldError('address', 'Address should be hex (256 bits)');

    api.program
      .exists(address)
      .then((isProgramExists) => {
        if (!isProgramExists) throw new Error('Program not found in the storage');

        setContractAddress(address);
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
