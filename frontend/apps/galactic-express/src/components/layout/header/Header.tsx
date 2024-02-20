import { Link } from 'react-router-dom';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { ReactComponent as GalexSVG } from 'assets/images/logo.svg';
import { ReactComponent as VaraSVG } from 'assets/images/logo-vara.svg';
import { cx } from 'utils';
import styles from './Header.module.scss';
import { useLaunchState } from 'features/session';
import { useAccount } from '@gear-js/react-hooks';
import { CancelGameButton } from 'features/session/components/cancel-game-button';

function Header() {
  const { account } = useAccount();
  const state = useLaunchState();
  const { admin, stage } = state || {};

  const isUserAdmin = admin === account?.decodedAddress;
  const isRegistration = Object.keys(stage || {})[0] === 'Registration';
  const participants = stage?.Registration || stage?.Results?.participants;

  return (
    <CommonHeader
      logo={
        <Link to="/">
          <VaraSVG className={cx(styles['vara-logo'])} />
          <GalexSVG />
        </Link>
      }
      menu={
        <MenuHandler
          className={{
            wallet: {
              balance: styles.walletBalance,
            },
            icon: styles.menuIcon,
          }}
        />
      }
      className={{ header: styles.header, content: styles.container }}>
      {isUserAdmin && isRegistration && <CancelGameButton isAdmin={isUserAdmin} participants={participants || []} />}
    </CommonHeader>
  );
}

export { Header };
