import { Link } from 'react-router-dom';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { ReactComponent as VaraSVG } from 'assets/images/icons/logo-vara.svg';
import { ReactComponent as CrossSVG } from 'assets/images/icons/cross-icon.svg';
import { Button } from '@gear-js/vara-ui';
import { useReadGameSessionState, useSyndoteMessage } from 'hooks/metadata';
import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { useQuitGame } from 'hooks/useQuitGame';
import styles from './Header.module.scss';

function Header() {
  const { state } = useReadGameSessionState();
  const { account } = useAccount();
  const { adminId, gameStatus } = state || {};
  const isAdmin = account?.decodedAddress === adminId;
  const { cancelGame, deleteGame, exitGame } = useQuitGame();

  return (
    <CommonHeader
      logo={
        <Link to="/">
          <VaraSVG className={styles['vara-logo']} />
        </Link>
      }
      menu={
        <div className={styles.headerContent}>
          {account?.decodedAddress && adminId === account?.decodedAddress && gameStatus !== 'Finished' && (
            <Button
              color="light"
              text="Cancel game"
              icon={CrossSVG}
              className={styles.cancelGameButton}
              onClick={cancelGame}
            />
          )}
          {account?.decodedAddress && gameStatus === 'Finished' && (
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
