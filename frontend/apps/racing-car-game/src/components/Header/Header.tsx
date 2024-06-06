import { EzGaslessTransactions, EzSignlessTransactions } from '@dapps-frontend/ez-transactions';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/consts';
import styles from './Header.module.scss';
import { Logo } from './logo';

function Header() {
  return (
    <CommonHeader
      logo={<Logo label="Racing Car" />}
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

export { Header };
