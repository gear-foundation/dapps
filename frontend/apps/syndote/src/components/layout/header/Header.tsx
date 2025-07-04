import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { Link } from 'react-router-dom';

import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';

import { useGetGameSessionQuery } from '@/app/utils';
import CrossSVG from '@/assets/images/icons/cross-icon.svg?react';
import VaraSVG from '@/assets/images/icons/logo-vara.svg?react';
import { useQuitGame } from '@/hooks/useQuitGame';

import styles from './Header.module.scss';

function Header() {
  const { account } = useAccount();
  const { state } = useGetGameSessionQuery();
  const { admin_id, game_status } = state || {};
  const isAdmin = account?.decodedAddress === admin_id;
  const { cancelGame, deleteGame, exitGame } = useQuitGame();
  const isFinished = game_status && 'finished' in game_status;

  return (
    <CommonHeader
      logo={
        <Link to="/">
          <VaraSVG className={styles['vara-logo']} />
        </Link>
      }
      menu={
        <div className={styles.headerContent}>
          {account?.decodedAddress && admin_id === account?.decodedAddress && isFinished && (
            <Button
              color="border"
              text="Cancel game"
              icon={CrossSVG}
              className={styles.cancelGameButton}
              onClick={cancelGame}
            />
          )}
          {account?.decodedAddress && isFinished && (
            <>
              {isAdmin ? (
                <Button text="Remove game" onClick={deleteGame} />
              ) : (
                <Button text="Leave game" onClick={exitGame} />
              )}
            </>
          )}
          <MenuHandler className={{ icon: styles.menuIcon }} />
        </div>
      }
      className={{ header: styles.header, content: styles.container }}
    />
  );
}

export { Header };
