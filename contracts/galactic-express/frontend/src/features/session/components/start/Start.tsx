import { useAccount, withoutCommas } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { Container } from 'components';
import { useLaunchState } from '../../hooks';
import { Wallet } from '../../../wallet';
import { Traits } from '../traits';
import { Form } from '../form';
import styles from './Start.module.scss';

function Start() {
  const { account } = useAccount();

  const state = useLaunchState();
  const { sessionId, currentSession } = state || {};
  const { registered } = currentSession || {};
  const players = Object.keys(registered || {});
  const playersCount = players.length;

  const containerClassName = clsx(styles.container, account ? styles.smallMargin : styles.largeMargin);

  console.log(state);

  return currentSession ? (
    <div>
      <header className={styles.header}>
        <h2 className={styles.heading}>Session #{sessionId}</h2>

        <div>
          <p className={styles.registration}>Registration...</p>
          <p className={styles.subheading}>Rockets ({playersCount}/4). Waiting for other players...</p>
        </div>
      </header>

      <Container className={containerClassName}>
        <Traits
          altitude={currentSession.altitude}
          weather={currentSession.weather}
          fuelPrice={currentSession.fuelPrice}
          reward={currentSession.reward}
        />

        <footer>
          {account ? (
            <Form weather={currentSession.weather} defaultDeposit={withoutCommas(currentSession.bet || '0')} />
          ) : (
            <div className={styles.wallet}>
              <Wallet />
              <p>Connect wallet to start calculation and launch</p>
            </div>
          )}
        </footer>
      </Container>
    </div>
  ) : (
    <p>Waiting for session to start...</p>
  );
}

export { Start };
