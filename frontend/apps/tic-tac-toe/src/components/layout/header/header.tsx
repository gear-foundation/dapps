import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { EzGaslessTransactions, EzSignlessTransactions } from 'gear-ez-transactions';

import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';

import styles from './header.module.scss';
import { Logo } from './logo';

export function Header() {
  const { account } = useAccount();

  return (
    <CommonHeader
      logo={
        <Logo className={clsx(styles.header__logo, !account && styles['header__logo--center'])} label="Tic-Tac-Toe" />
      }
      className={{ header: styles.header, content: styles.header__container }}
      menu={
        <MenuHandler
          customItems={[
            { key: 'signless', option: <EzSignlessTransactions allowedActions={SIGNLESS_ALLOWED_ACTIONS} /> },
            { key: 'gasless', option: <EzGaslessTransactions /> },
          ]}
        />
      }
    />
  );
}
