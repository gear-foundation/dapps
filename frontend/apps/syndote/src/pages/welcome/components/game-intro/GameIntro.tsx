import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';

import styles from './GameIntro.module.scss';

type Props = {
  heading: string;
  textColor?: 'white' | 'black';
  status: 'creating' | 'joining' | 'loading' | null;
};

function GameIntro({ heading, status, textColor = 'black' }: Props) {
  const { account } = useAccount();

  return (
    <div className={styles.container}>
      <h1 className={clsx(styles.name, styles[`name-${textColor}`])}>{heading}</h1>
      <p className={clsx(styles.description, styles[`description-${textColor}`])}>
        {account?.decodedAddress ? (
          <>
            {!status &&
              'To begin, choose whether you want to join an existing game or become an administrator and create a new game.'}
            {status === 'creating' &&
              'Set the entry fee. After creating the game, share your unique game ID (which is your wallet address) so players can join.'}
            {status === 'joining' &&
              'To start the game, enter the game admin address you received from the administrator'}
          </>
        ) : (
          'Connect your wallet to start'
        )}
      </p>
    </div>
  );
}

export { GameIntro };
