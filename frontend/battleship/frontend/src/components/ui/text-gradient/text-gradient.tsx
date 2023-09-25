import styles from './text-gradient.module.scss';

export function TextGradient({ children }: React.PropsWithChildren) {
  return <span className={styles.gradient}>{children}</span>;
}
