import styles from '../columns.module.scss';

type ContainerProps = BaseComponentProps & {};

export function ColumnsContainer({ children }: ContainerProps) {
  return <div className={styles.container}>{children}</div>;
}
