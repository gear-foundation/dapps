import { Link } from 'react-router-dom';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { ReactComponent as VaraSVG } from 'assets/images/icons/logo-vara.svg';
import { ReactComponent as CrossSVG } from 'assets/images/icons/cross-icon.svg';
import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import { useQuitGame } from 'hooks/useQuitGame';
import { useGetGameSessionQuery } from 'app/utils';
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
              color="light"
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
