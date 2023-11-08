import { NavLink } from 'react-router-dom';
import clsx from 'clsx';
import styles from './logo.module.scss';
import { ROUTES } from '@/app/consts';
import { TextGradient } from '@/components/ui/text-gradient';
import { Sprite } from '@/components/ui/sprite';
import type { BaseComponentProps } from '@/app/types';

type LogoProps = BaseComponentProps & {
  label?: string;
};

export function Logo({ className, label }: LogoProps) {
  return (
    <NavLink to={ROUTES.HOME} className={({ isActive }) => clsx(styles.link, isActive && styles.active, className)}>
      <Sprite name="vara-logo" width={92} height={60} className={styles.logo} />
      {label && <TextGradient className={styles.title}>{label}</TextGradient>}
    </NavLink>
  );
}
