import { Input, Button } from '@gear-js/ui';
import { useState, ChangeEvent } from 'react';
import { Card } from 'components';
import { ReactComponent as RocketSVG } from '../../assets/rocket.svg';
import { Range } from '../range';
import styles from './Form.module.scss';

function Form() {
  const [payload, setPayload] = useState('0');
  const [fuel, setFuel] = useState('0');

  const handlePayloadChange = ({ target }: ChangeEvent<HTMLInputElement>) => setPayload(target.value);
  const handleFuelChange = ({ target }: ChangeEvent<HTMLInputElement>) => setFuel(target.value);

  return (
    <form>
      <Card className={styles.deposit}>
        <h3 className={styles.heading}>Mission Deposit</h3>
        <Input label="Deposit (VARA):" className={styles.input} />
      </Card>

      <Card className={styles.calculation}>
        <h3 className={styles.heading}>Calculation Block</h3>

        <div className={styles.ranges}>
          <Range label="Payload:" value={payload} onChange={handlePayloadChange} />
          <Range label="Fuel:" value={fuel} onChange={handleFuelChange} />
        </div>

        <footer className={styles.footer}>
          <p className={styles.probability}>
            Success Probability:
            <span className={styles.value}>50%</span>
          </p>

          <Button type="submit" icon={RocketSVG} text="Launch Rocket" color="lightGreen" />
        </footer>
      </Card>
    </form>
  );
}

export { Form };
