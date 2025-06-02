import { useAccount } from '@gear-js/react-hooks';
import { Link } from 'react-router-dom';

import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';

import VaraSVG from '@/assets/images/logo-vara.svg?react';
import GalexSVG from '@/assets/images/logo.svg?react';
import { useLaunchState } from '@/features/session';
import { CancelGameButton } from '@/features/session/components/cancel-game-button';
import { cx } from '@/utils';

import styles from './Header.module.scss';

function Header() {
  const { account } = useAccount();
  const state = useLaunchState();
  const { admin, stage } = state || {};

  const isUserAdmin = admin === account?.decodedAddress;
  const isRegistration = stage && 'registration' in stage;
  const participants = isRegistration ? stage.registration : [];

  return (
    <CommonHeader
      logo={
        <Link to="/">
          <VaraSVG className={cx(styles['vara-logo'])} />
          <GalexSVG />
        </Link>
      }
      menu={<MenuHandler className={{ icon: styles.menuIcon }} />}
      className={{ header: styles.header, content: styles.container }}>
      {isUserAdmin && isRegistration && <CancelGameButton isAdmin={isUserAdmin} participants={participants} />}
    </CommonHeader>
  );
}

export { Header };
