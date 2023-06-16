import { useAccount, withoutCommas } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { Container } from 'components';
import { Session } from '../../types';
import { Wallet } from '../../../wallet';
import { Traits } from '../traits';
import { Form } from '../form';
import styles from './Start.module.scss';

type Props = {
  sessionId: string;
  session: Session;
};

function Start({ sessionId, session }: Props) {
  const { account } = useAccount();

  const { registered, altitude, weather, fuelPrice, reward, bet } = session;
  const players = Object.keys(registered || {});
  const playersCount = players.length;

  const containerClassName = clsx(styles.container, account ? styles.smallMargin : styles.largeMargin);

  return (
    <div>
      <header className={styles.header}>
        <h2 className={styles.heading}>Session #{sessionId}</h2>

        <div>
          <p className={styles.registration}>Registration...</p>
          <p className={styles.subheading}>Rockets ({playersCount}/4). Waiting for other players...</p>
        </div>
      </header>

      <Container className={containerClassName}>
        <Traits altitude={altitude} weather={weather} fuelPrice={fuelPrice} reward={reward} />

        <footer>
          {account ? (
            <Form weather={weather} defaultDeposit={withoutCommas(bet || '0')} />
          ) : (
            <div className={styles.wallet}>
              <Wallet />
              <p>Connect wallet to start calculation and launch</p>
            </div>
          )}
        </footer>
      </Container>
    </div>
  );
}

export { Start };
