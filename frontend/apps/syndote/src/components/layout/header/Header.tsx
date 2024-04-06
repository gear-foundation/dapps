import { Link } from 'react-router-dom';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { ReactComponent as VaraSVG } from 'assets/images/icons/logo-vara.svg';
import { ReactComponent as CrossSVG } from 'assets/images/icons/cross-icon.svg';
import styles from './Header.module.scss';
import { Button } from '@gear-js/vara-ui';
import { useReadGameSessionState, useSyndoteMessage } from 'hooks/metadata';
import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';

function Header() {
  const { state, isStateRead } = useReadGameSessionState();
  const { isMeta, sendMessage } = useSyndoteMessage();
  const { account } = useAccount();
  const { adminId, gameStatus } = state || {};
  const isAdmin = account?.decodedAddress === adminId;

  const handleCancelGame = () => {
    if (!isMeta || !account?.decodedAddress || !isStateRead) {
      return;
    }

    const payload = {
      CancelGameSession: {
        adminId,
      },
    };

    sendMessage({
      payload,
    });
  };

  const exitGame = () => {
    const payload = {
      ExitGame: {
        adminId,
      },
    };

    sendMessage({
      payload,
    });
  };

  const deleteGame = () => {
    const payload = {
      DeleteGame: {
        adminId,
      },
    };

    sendMessage({
      payload,
    });
  };

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
              onClick={handleCancelGame}
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
          <MenuHandler
            className={{
              wallet: {
                balance: styles.walletBalance,
              },
              icon: styles.menuIcon,
            }}
          />
        </div>
      }
      className={{ header: styles.header, content: styles.container }}></CommonHeader>
  );
}

export { Header };
