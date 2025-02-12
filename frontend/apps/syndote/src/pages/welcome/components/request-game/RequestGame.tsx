import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { useAtom } from 'jotai';
import { useState } from 'react';

import { Wallet } from '@dapps-frontend/ui';

import { IS_LOADING } from '@/atoms';

import { CreateGameForm } from '../create-game-form';
import { GameIntro } from '../game-intro';
import { JoinGameForm } from '../join-game-form';

import styles from './RequestGame.module.scss';

type Status = 'creating' | 'joining' | null;

function RequestGame() {
  const { account } = useAccount();
  const [status, setStatus] = useState<Status>(null);
  const [isLoading] = useAtom(IS_LOADING);

  const handleSetStatus = (newStatus: Status) => {
    setStatus(newStatus);
  };

  // useEffect(() => {
  //   setRegistrationStatus('registration');

  //   // eslint-disable-next-line react-hooks/exhaustive-deps
  // }, []);

  return (
    <div className={styles.container}>
      <GameIntro heading="Welcome to Syndote" status={status} />
      {account ? (
        <>
          {!status && (
            <div className={styles.controls}>
              <Button
                type="submit"
                text="Find game"
                className={styles.button}
                onClick={() => handleSetStatus('joining')}
                disabled={isLoading}
              />
              <Button
                type="submit"
                text="Create game"
                color="dark"
                className={styles.button}
                onClick={() => handleSetStatus('creating')}
                disabled={isLoading}
              />
            </div>
          )}
          {status === 'creating' && <CreateGameForm onCancel={() => handleSetStatus(null)} />}
          {status === 'joining' && <JoinGameForm onCancel={() => handleSetStatus(null)} />}
        </>
      ) : (
        <Wallet />
      )}
    </div>
  );
}

export { RequestGame };
