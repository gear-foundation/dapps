import { EzGaslessTransactions, EzSignlessTransactions } from '@dapps-frontend/ez-transactions';
import { Logo } from './logo';
import styles from './header.module.scss';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import clsx from 'clsx';
import { useAccount } from '@gear-js/react-hooks';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';

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
