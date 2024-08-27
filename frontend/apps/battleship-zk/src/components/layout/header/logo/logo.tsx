import { NavLink } from 'react-router-dom';
import clsx from 'clsx';
import styles from './logo.module.scss';
import { VaraLogoIcon } from '@/assets/images';
import { ROUTES } from '@/app/consts';

export function Logo({ className }: BaseComponentProps) {
  return (
    <NavLink to={ROUTES.HOME} className={({ isActive }) => clsx(styles.link, isActive && styles.active, className)}>
      <VaraLogoIcon className={styles.logo} />
    </NavLink>
  );
}
