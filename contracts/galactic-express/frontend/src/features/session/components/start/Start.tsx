import { useSessionState } from 'features/session/hooks';
import { TRAITS } from 'features/session/consts';
import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { Wallet } from '../../../wallet';
import { Trait } from '../trait';
import { Form } from '../form';
import styles from './Start.module.scss';

function Start() {
  const { account } = useAccount();

  const state = useSessionState();
  const { sessionId, currentSession } = state || {};

  const heading = `Session #${sessionId}`;
  const playersCount = 0;
  const subheading = `Rockets (${playersCount}/4). Waiting for other players...`;

  const containerClassName = clsx(styles.container, account ? styles.smallMargin : styles.largeMargin);

  const getTraits = () => {
    if (!currentSession) return;

    const { altitude, weather, fuelPrice, reward } = currentSession || {};

    // same order as in TRAITS
    const traitValues = [altitude, weather, fuelPrice, reward];

    return TRAITS.map((trait, index) => (
      <Trait key={trait.heading} SVG={trait.SVG} heading={trait.heading} subheading={traitValues[index]} />
    ));
  };

  console.log(state);

  return currentSession ? (
    <div>
      <header className={styles.header}>
        <h2 className={styles.heading}>{heading}</h2>

        <div>
          <p className={styles.registration}>Registration...</p>
          <p className={styles.subheading}>{subheading}</p>
        </div>
      </header>

      <div className={containerClassName}>
        <ul className={styles.traits}>{getTraits()}</ul>

        <footer>
          {account ? (
            <Form weather={currentSession.weather} />
          ) : (
            <div className={styles.wallet}>
              <Wallet />
              <p>Connect wallet to start calculation and launch</p>
            </div>
          )}
        </footer>
      </div>
    </div>
  ) : (
    <p>Waiting for session to start...</p>
  );
}

export { Start };
