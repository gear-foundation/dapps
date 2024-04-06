import { cx } from 'utils';
import { useAccount } from '@gear-js/react-hooks';
import styles from './GameIntro.module.scss';

type Props = {
  status: 'creating' | 'joining' | 'loading' | null;
};

function GameIntro({ status }: Props) {
  const { account } = useAccount();
  return (
    <div className={cx(styles.container)}>
      <h1 className={cx(styles.name)}>Welcome to Galactic Express</h1>
      <p className={cx(styles.description)}>
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
