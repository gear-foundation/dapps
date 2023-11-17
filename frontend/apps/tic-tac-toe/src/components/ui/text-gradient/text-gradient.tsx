import styles from './text-gradient.module.scss';
import clsx from 'clsx';

export function TextGradient({ children, className }: React.PropsWithChildren & { className?: string }) {
  return <span className={clsx(styles.gradient, className)}>{children}</span>;
}
