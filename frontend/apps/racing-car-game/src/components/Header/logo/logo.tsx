import styles from './logo.module.scss';
import logo from '@/assets/icons/logo-vara-black.svg';
import { Link } from '@/ui';

type LogoProps = {
  label?: string;
  className?: string;
};

export function Logo({ label, className }: LogoProps) {
  return (
    <Link to="/" className={className}>
      <img src={logo} alt="" />
      <span className={styles['post-logo']}>{label}</span>
    </Link>
  );
}
