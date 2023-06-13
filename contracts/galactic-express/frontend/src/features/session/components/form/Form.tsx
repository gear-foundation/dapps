import { Input, Button } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { Card } from 'components';
import clsx from 'clsx';
import { ReactComponent as RocketSVG } from '../../assets/rocket.svg';
import { useProbability } from '../../hooks';
import { Range } from '../range';
import styles from './Form.module.scss';

const initialValues = {
  deposit: '0',
  payload: '0',
  fuel: '0',
};

type Props = {
  weather: string;
};

function Form({ weather }: Props) {
  const { getInputProps, values } = useForm({ initialValues });
  const { payload, fuel } = values;

  const { probability, probabilityId } = useProbability(weather, payload, fuel);
  const probabilityValueClassName = clsx(styles.value, styles[probabilityId]);

  return (
    <form>
      <Card className={styles.deposit}>
        <h3 className={styles.heading}>Mission Deposit</h3>
        <Input type="number" label="Deposit (VARA):" className={styles.input} {...getInputProps('deposit')} />
      </Card>

      <Card className={styles.calculation}>
        <h3 className={styles.heading}>Calculation Block</h3>

        <div className={styles.ranges}>
          <Range label="Payload:" {...getInputProps('payload')} />
          <Range label="Fuel:" {...getInputProps('fuel')} />
        </div>

        <footer className={styles.footer}>
          <p className={styles.probability}>
            Success Probability:
            <span className={probabilityValueClassName}>{probability}%</span>
          </p>

          <Button type="submit" icon={RocketSVG} text="Launch Rocket" color="lightGreen" />
        </footer>
      </Card>
    </form>
  );
}

export { Form };
