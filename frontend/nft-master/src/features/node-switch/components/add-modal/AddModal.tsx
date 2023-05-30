import { Button, Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { Modal } from 'components';
import { NodeSection } from '../../types';
import { isNodeAddressValid, isNodeExists } from '../../utils';
import styles from './AddModal.module.scss';

type Props = {
  sections: NodeSection[];
  onClose: () => void;
  onSubmit: (value: string) => void;
};

const initialValues = { address: '' };

function AddModal({ sections, onClose, onSubmit }: Props) {
  const form = useForm({ initialValues });
  const { getInputProps, setFieldError } = form;

  const handleSubmit = form.onSubmit(({ address }) => {
    if (!isNodeAddressValid(address)) return setFieldError('address', 'Address not valid');
    if (isNodeExists(sections, address)) return setFieldError('address', 'Address already exists');

    onSubmit(address);
    onClose();
  });

  return (
    <Modal heading="Add Network" onClose={onClose}>
      <form className={styles.form} onSubmit={handleSubmit}>
        <Input placeholder="wss://address.rs" {...getInputProps('address')} />

        <Button type="submit" text="OK" block />
      </form>
    </Modal>
  );
}

export { AddModal };
