import clsx from 'clsx';
import { NavLink } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { VaraLogoIcon } from '@/assets/images';

import styles from './logo.module.scss';

export function Logo({ className }: BaseComponentProps) {
  return (
    <NavLink to={ROUTES.HOME} className={({ isActive }) => clsx(styles.link, isActive && styles.active, className)}>
      <VaraLogoIcon className={styles.logo} />
    </NavLink>
  );
}
