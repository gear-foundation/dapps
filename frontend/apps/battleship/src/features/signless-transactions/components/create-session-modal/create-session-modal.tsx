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
  value: '',
  duration: '',
  password: '',
};

const REQUIRED_MESSAGE = 'Field is required';

const ACTIONS = ['StartGame', 'Turn'];

function CreateSessionModal({ close }: Props) {
  const { api } = useApi();
  const { getChainBalanceValue, getFormattedBalanceValue } = useBalanceFormat();
  // TODO: omit type after @gear-js/react-hooks BigNumber.js types fix
  const eDeposit: string = getFormattedBalanceValue(api?.existentialDeposit.toString() || '0').toFixed();
  const [unit] = api?.registry.chainTokens || ['Unit'];

  const { register, handleSubmit, formState } = useForm({ defaultValues: DEFAULT_VALUES });
  const { errors } = formState;

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
          <Input
            type="number"
            label={`Value (${unit})`}
            error={errors.value?.message}
            {...register('value', {
              required: REQUIRED_MESSAGE,
              min: { value: eDeposit, message: `Minimum value is ${eDeposit}` },
            })}
          />

          <Input
            type="number"
            label="Duration (minutes)"
            error={errors.duration?.message}
            {...register('duration', {
              required: REQUIRED_MESSAGE,
              min: { value: 1, message: 'Minimum value is 1' },
            })}
          />

          <Input
            type="password"
            label="Password"
            error={errors.password?.message}
            {...register('password', {
              required: REQUIRED_MESSAGE,
              minLength: { value: 6, message: 'Minimum length is 6' },
            })}
          />
        </div>

        <Button type="submit" text="Submit" className={styles.button} />
      </form>
    </Modal>
  );
}

export { CreateSessionModal };
