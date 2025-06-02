import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { EzGaslessTransactions, EzSignlessTransactions } from 'gear-ez-transactions';

import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { useGame } from '@/app/context/ctx-game';
import { HeaderAdmin } from '@/components/layout/header/header-admin';

import styles from './Header.module.scss';
import { Logo } from './logo';

export const Header = () => {
  const { isAdmin } = useGame();
  const { account } = useAccount();

  return (
    <CommonHeader
      logo={<Logo className={clsx(styles.header__logo, !account && styles['header__logo--center'])} />}
      menu={
        <MenuHandler
          customItems={[
            { key: 'signless', option: <EzSignlessTransactions allowedActions={SIGNLESS_ALLOWED_ACTIONS} /> },
            { key: 'gasless', option: <EzGaslessTransactions /> },
          ]}
        />
      }
      className={{ header: styles.header, content: styles.header__container }}>
      {isAdmin && <HeaderAdmin />}
    </CommonHeader>
  );
};
