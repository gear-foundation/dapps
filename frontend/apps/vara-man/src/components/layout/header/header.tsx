import { useGame } from '@/app/context/ctx-game';
import { HeaderAdmin } from '@/components/layout/header/header-admin';

import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { EzGaslessTransactions, EzSignlessTransactions } from 'gear-ez-transactions';

import styles from './Header.module.scss';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { Logo } from './logo';
import clsx from 'clsx';
import { useAccount } from '@gear-js/react-hooks';

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
