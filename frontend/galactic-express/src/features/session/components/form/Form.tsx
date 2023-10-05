import { useAtomValue } from 'jotai';
import { Input, Button } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { Card } from 'components';
import { CURRENT_CONTRACT_ADDRESS_ATOM } from 'atoms';
import { ChangeEvent, Dispatch, SetStateAction } from 'react';
import { ReactComponent as RocketSVG } from '../../assets/rocket.svg';
import { INITIAL_VALUES, VALIDATE, WEATHERS } from '../../consts';
import { useLaunchMessage } from '../../hooks';
import { Range } from '../range';
import { Probability } from '../probability';
import styles from './Form.module.scss';

type Props = {
  weather: string;
  defaultDeposit: string;
  isAdmin: boolean;
  setRegistrationStatus: Dispatch<
    SetStateAction<'registration' | 'success' | 'error' | 'NotEnoughParticipants' | 'MaximumPlayersReached'>
  >;
};

function Form({ weather, defaultDeposit, isAdmin, setRegistrationStatus }: Props) {
  const currentContractAddress = useAtomValue(CURRENT_CONTRACT_ADDRESS_ATOM);
  const { values, getInputProps, onSubmit, setFieldValue } = useForm({
    initialValues: { deposit: defaultDeposit, ...INITIAL_VALUES },
    validate: VALIDATE,
  });

  const { fuel, payload, deposit } = values;

  const { meta, message: sendMessage } = useLaunchMessage(currentContractAddress);

  const isFirstPlayer = defaultDeposit === '0';

  const handleNumberInputChange = ({ target }: ChangeEvent<HTMLInputElement>) => {
    const value = +target.value;
    const min = +target.min || -Infinity;
    const max = +target.max || Infinity;

    const rangeValue = Math.max(min, Math.min(max, value));

    setFieldValue(target.name, String(rangeValue));
  };

  const getNumberInputProps = (name: keyof typeof values) => ({
    ...getInputProps(name),
    name, // passing name cuz getInputProps doesn't do this
    onChange: handleNumberInputChange,
  });

  const handleSubmit = () => {
    if (!isAdmin && meta) {
      sendMessage(
        { Register: { fuel_amount: fuel, payload_amount: payload } },
        {
          onSuccess: () => {
            setRegistrationStatus('success');
          },
        },
      );
    }

    if (isAdmin && meta) {
      sendMessage({ StartGame: { fuel_amount: fuel, payload_amount: payload } });
    }
  };

  return (
    <form onSubmit={onSubmit(handleSubmit)}>
      <Card className={styles.deposit}>
        <h3 className={styles.heading}>Mission Deposit</h3>
        <Input
          type="number"
          label="Deposit (VARA):"
          className={styles.input}
          min={0}
          readOnly={!isFirstPlayer}
          {...getNumberInputProps('deposit')}
        />
      </Card>

      <Card className={styles.calculation}>
        <h3 className={styles.heading}>Calculation Block</h3>

        <div className={styles.ranges}>
          <Range key="first_range" label="Payload:" {...getNumberInputProps('payload')} />
          <Range key="second_range" label="Fuel:" {...getNumberInputProps('fuel')} />
        </div>

        <footer className={styles.footer}>
          <Probability weather={WEATHERS[weather as keyof typeof WEATHERS].weight} payload={+payload} fuel={+fuel} />
          <Button
            type="submit"
            icon={RocketSVG}
            text={isAdmin ? 'Launch rocket and start Game' : 'Launch Rocket'}
            color="lightGreen"
          />
        </footer>
      </Card>
    </form>
  );
}

export { Form };
