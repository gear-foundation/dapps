import styles from './loading-error.module.scss';

type LoadingErrorProps = BaseComponentProps & {};

export function LoadingError({ children }: LoadingErrorProps) {
  return <div className={styles.box}>{children}</div>;
}
