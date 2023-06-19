import { useAccount, withoutCommas } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { Container } from 'components';
import { Auth } from '../../../auth';
import src from '../../assets/earth.gif';
import { Session } from '../../types';
import { Traits } from '../traits';
import { Form } from '../form';
import styles from './Start.module.scss';

type Props = {
  sessionId: string;
  session: Session;
};

function Start({ sessionId, session }: Props) {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { registered, altitude, weather, fuelPrice, reward, bet } = session;
  const players = Object.keys(registered);
  const playersCount = players.length;
  const isRegistered = decodedAddress ? !!registered[decodedAddress] : false;

  const containerClassName = clsx(styles.container, account ? styles.smallMargin : styles.largeMargin);

  return (
    <div className={styles.mainContainer}>
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
              <>
                {isRegistered && <p>You&apos;re registered.</p>}
                {!isRegistered && <Form weather={weather} defaultDeposit={withoutCommas(bet || '0')} />}
              </>
            ) : (
              <div className={styles.wallet}>
                <Auth hideResetButton />
                <p>Connect wallet to start calculation and launch</p>
              </div>
            )}
          </footer>
        </Container>
      </div>

      <div className={styles.imageWrapper}>
        <img src={src} alt="" className={styles.image} />
      </div>
    </div>
  );
}

export { Start };
