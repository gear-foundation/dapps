import { Input, Button } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { Card } from 'components';
import { ChangeEvent } from 'react';
import { ReactComponent as RocketSVG } from '../../assets/rocket.svg';
import { INITIAL_VALUES, VALIDATE } from '../../consts';
import { useLaunchMessage } from '../../hooks';
import { Range } from '../range';
import { Probability } from '../probability';
import styles from './Form.module.scss';

type Props = {
  weather: string;
  defaultDeposit: string;
};

function Form({ weather, defaultDeposit }: Props) {
  const { values, getInputProps, onSubmit, setFieldValue } = useForm({
    initialValues: { deposit: defaultDeposit, ...INITIAL_VALUES },
    validate: VALIDATE,
  });

  const { fuel, payload, deposit } = values;

  const sendMessage = useLaunchMessage();

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

  const handleSubmit = onSubmit(() =>
    sendMessage({ RegisterParticipantOnLaunch: { fuel_amount: fuel, payload_amount: payload } }, { value: deposit }),
  );

  return (
    <form onSubmit={handleSubmit}>
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
          <Probability weather={+weather} payload={+payload} fuel={+fuel} />
          <Button type="submit" icon={RocketSVG} text="Launch Rocket" color="lightGreen" />
        </footer>
      </Card>
    </form>
  );
}

export { Form };
