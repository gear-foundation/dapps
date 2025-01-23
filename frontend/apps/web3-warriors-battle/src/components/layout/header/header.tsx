import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { EzSignlessTransactions, EzGaslessTransactions } from 'gear-ez-transactions';
import { useNavigate } from 'react-router-dom';

import { ALLOWED_SIGNLESS_ACTIONS, ROUTES } from '@/app/consts';
import { GameButton } from '@/features/game/components/game-button';

import { Logo } from './logo';
import styles from './header.module.scss';

export function Header() {
  const { account } = useAccount();
  const navigate = useNavigate();

  return (
    <CommonHeader
      logo={
        <div className={styles.header__logoWrapper}>
          <Logo className={clsx(styles.header__logo, !account && styles['header__logo--center'])} />
          <GameButton
            color="black"
            text="Show tutorial"
            className={styles.header__tutorial}
            onClick={() => navigate(ROUTES.ONBOARDING)}
          />
        </div>
      }
      className={{ header: styles.header, content: styles.header__container }}
      menu={
        <MenuHandler
          customItems={[
            { key: 'signless', option: <EzSignlessTransactions allowedActions={ALLOWED_SIGNLESS_ACTIONS} /> },
            { key: 'gasless', option: <EzGaslessTransactions /> },
          ]}
        />
      }
    />
  );
}
