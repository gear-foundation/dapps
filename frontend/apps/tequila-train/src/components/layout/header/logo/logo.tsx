import clsx from 'clsx';
import { NavLink } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import type { BaseComponentProps } from '@/app/types';
import { Sprite } from '@/components/ui/sprite';

import styles from './logo.module.scss';

type LogoProps = BaseComponentProps & {
  label?: string;
};

export function Logo({ className }: LogoProps) {
  return (
    <NavLink to={ROUTES.HOME} className={({ isActive }) => clsx(styles.link, isActive && styles.active, className)}>
      <Sprite name="vara-logo" width={92} height={60} className={styles.logo} />
      {/* {label && <TextGradient className={styles.title}>{label}</TextGradient>} */}
    </NavLink>
  );
}
