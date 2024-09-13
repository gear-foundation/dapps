import { Logo } from './logo';
import styles from './header.module.scss';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import clsx from 'clsx';
import { useAccount } from '@gear-js/react-hooks';

export function Header() {
  const { account } = useAccount();

  return (
    <CommonHeader
      logo={<Logo className={clsx(styles.header__logo, !account && styles['header__logo--center'])} />}
      className={{ header: styles.header, content: styles.header__container }}
      menu={<MenuHandler />}
    />
  );
}
