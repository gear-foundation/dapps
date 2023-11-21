import { useApi } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { HexString } from '@polkadot/util/types';
import { Content, Input } from 'components';
import { isValidHex } from 'utils';
import styles from './Use.module.scss';

type Props = {
  onCancel: () => void;
  onSubmit: (value: HexString) => void;
};

const initialValues = { programId: '' as HexString };
const validate = { programId: isValidHex };

function Use({ onCancel, onSubmit }: Props) {
  const { api } = useApi();
  const form = useForm({ initialValues, validate });
  const { getInputProps, values, setFieldError } = form;

  const handleSubmit = form.onSubmit(async ({ programId }) => {
    const isProgramExists = await api?.program.exists(values.programId);

    if (isProgramExists) {
      onSubmit(programId);
    } else setFieldError('programId', 'Program not found in the storage');
  });

  return (
    <Content
      heading="Type here the ID of an existing supply chain program 
  and click “Login” to continue."
      className={styles.content}>
      <form onSubmit={handleSubmit}>
        <div className={styles.box}>
          <Input label="Program ID" className={styles.input} {...getInputProps('programId')} />
        </div>
        <div className={styles.buttons}>
          <Button text="Cancel" color="secondary" onClick={onCancel} />
          <Button type="submit" text="Submit" />
        </div>
      </form>
    </Content>
  );
}

export { Use };
