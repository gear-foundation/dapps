import { Button, Input, Modal, ModalProps, Textarea } from '@gear-js/vara-ui';
import { ChangeEvent, FormEvent, useState } from 'react';

import styles from './create-session-modal.module.css';
import { useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { ADDRESS } from '@/app/consts';
import { useCreateSession, useIssueVoucher } from '../../hooks';
import { getMilliseconds, getRandomAccount } from '../../utils';
import { toJSON } from '@gear-js/api';

type Props = Pick<ModalProps, 'close'>;

const DEFAULT_VALUES = {
  value: '0',
  duration: '0',
  actions: '[ "StartGame" ]',
  password: '',
};

function CreateSessionModal({ close }: Props) {
  const { api } = useApi();
  const { getChainBalanceValue } = useBalanceFormat();
  const [unit] = api?.registry.chainTokens || ['Unit'];

  const [values, setValues] = useState(DEFAULT_VALUES);

  const createSession = useCreateSession();
  const issueVoucher = useIssueVoucher();

  const onChange = ({ target }: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) =>
    setValues((prevValues) => ({ ...prevValues, [target.name]: target.value }));

  const getInputProps = (name: keyof typeof DEFAULT_VALUES) => ({ onChange, name, value: values[name] });

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const value = getChainBalanceValue(values.value);
    const duration = getMilliseconds(+values.duration);
    const actions = toJSON(values.actions);

    const { publicKey } = getRandomAccount();

    issueVoucher(ADDRESS.GAME, value, () => createSession(publicKey, duration, actions));
  };

  return (
    <Modal heading="Create Signless Session" close={close}>
      <form onSubmit={handleSubmit}>
        <div className={styles.inputs}>
          <Input type="number" label={`Value (${unit})`} {...getInputProps('value')} />
          <Input type="number" label="Duration (minutes)" {...getInputProps('duration')} />
          <Textarea label="Actions (JSON)" {...getInputProps('actions')} />
          <Input type="password" label="Password" {...getInputProps('password')} />
        </div>

        <Button type="submit" text="Submit" className={styles.button} />
      </form>
    </Modal>
  );
}

export { CreateSessionModal };
