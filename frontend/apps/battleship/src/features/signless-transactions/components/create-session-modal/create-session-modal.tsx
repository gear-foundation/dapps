import { Button, Input, Modal, ModalProps } from '@gear-js/vara-ui';
import { useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { decodeAddress } from '@gear-js/api';
import { useForm } from 'react-hook-form';

import { ADDRESS } from '@/app/consts';

import { useSignlessTransactions } from '../../context';
import { useCreateSession, useIssueVoucher } from '../../hooks';
import { getMilliseconds, getRandomPair } from '../../utils';
import styles from './create-session-modal.module.css';

type Props = Pick<ModalProps, 'close'>;

const DEFAULT_VALUES = {
  value: '0',
  duration: '0',
  password: '',
};

const ACTIONS = ['StartGame', 'Turn'];

function CreateSessionModal({ close }: Props) {
  const { api } = useApi();
  const { getChainBalanceValue } = useBalanceFormat();
  const [unit] = api?.registry.chainTokens || ['Unit'];

  const { register, handleSubmit } = useForm({ defaultValues: DEFAULT_VALUES });
  const { savePair } = useSignlessTransactions();

  const { createSession, deleteSession } = useCreateSession();
  const issueVoucher = useIssueVoucher();

  const onSubmit = (values: typeof DEFAULT_VALUES) => {
    const { password } = values;
    const value = getChainBalanceValue(values.value).toFixed();
    const duration = getMilliseconds(+values.duration);

    const pair = getRandomPair();
    const decodedAddress = decodeAddress(pair.address);

    const onSuccess = () => {
      savePair(pair, password);
      close();
    };

    createSession(decodedAddress, duration, ACTIONS, () =>
      issueVoucher(ADDRESS.GAME, decodedAddress, value, onSuccess, deleteSession),
    );
  };

  return (
    <Modal heading="Create Signless Session" close={close}>
      <form onSubmit={handleSubmit(onSubmit)}>
        <div className={styles.inputs}>
          <Input type="number" label={`Value (${unit})`} {...register('value')} />
          <Input type="number" label="Duration (minutes)" {...register('duration')} />
          <Input type="password" label="Password" {...register('password')} />
        </div>

        <Button type="submit" text="Submit" className={styles.button} />
      </form>
    </Modal>
  );
}

export { CreateSessionModal };
