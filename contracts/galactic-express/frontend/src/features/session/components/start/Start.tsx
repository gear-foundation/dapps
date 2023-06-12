import { useSessionState } from 'features/session/hooks';
import { TRAITS } from 'features/session/consts';
import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { Wallet } from '../../../wallet';
import { Trait } from '../trait';
import styles from './Start.module.scss';

function Start() {
  const { account } = useAccount();
  const state = useSessionState();

  const { sessionId, currentSession } = state || {};

  console.log(state);

  const heading = `Session #${sessionId}`;

  const playersCount = 0;
  const subheading = `Rockets (${playersCount}/4). Waiting for other players...`;

  const footerClassName = clsx(styles.footer, account ? styles.smallMargin : styles.largeMargin);

  const getTraits = () => {
    if (!currentSession) return;

    const { altitude, weather, fuelPrice, payloadValue } = currentSession || {};
    const traitValues = [altitude, weather, fuelPrice, payloadValue];

    return TRAITS.map((trait, index) => (
      <Trait SVG={trait.SVG} heading={trait.heading} subheading={traitValues[index]} />
    ));
  };

  return currentSession ? (
    <div>
      <header className={styles.header}>
        <h2 className={styles.heading}>{heading}</h2>

        <div>
          <p className={styles.registration}>Registration...</p>
          <p className={styles.subheading}>{subheading}</p>
        </div>
      </header>

      <ul className={styles.traits}>{getTraits()}</ul>

      <footer className={footerClassName}>
        {account ? null : (
          <div className={styles.wallet}>
            <Wallet />
            <p>Connect wallet to start calculation and launch</p>
          </div>
        )}
      </footer>
    </div>
  ) : (
    <p>Waiting for session to start...</p>
  );
}

export { Start };
