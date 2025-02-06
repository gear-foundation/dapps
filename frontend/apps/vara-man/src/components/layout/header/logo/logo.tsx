import clsx from 'clsx';
import { NavLink } from 'react-router-dom';

import { SpriteIcon } from '@/components/ui/sprite-icon';

import styles from './logo.module.scss';

// import { ROUTES } from '@/app/consts';
// import { TextGradient } from '@/components/ui/text-gradient';
// import { Sprite } from '@/components/ui/sprite';
// import type { BaseComponentProps } from '@/app/types';

type LogoProps = BaseComponentProps & {
  label?: string;
};

export function Logo({ className }: LogoProps) {
  return (
    <NavLink to={'/'} className={({ isActive }) => clsx(styles.link, isActive && styles.active, className)}>
      <SpriteIcon name="vara-logo" width={92} height={60} className={styles.logo} />
    </NavLink>
  );
}
