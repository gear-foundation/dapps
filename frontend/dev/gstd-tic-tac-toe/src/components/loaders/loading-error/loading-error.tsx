import styles from './loading-error.module.scss';
import { Container } from '@/components/ui/container';
import { BaseComponentProps } from '@/app/types';

type LoadingErrorProps = BaseComponentProps & {};

export function LoadingError({ children }: LoadingErrorProps) {
  return <Container className={styles.box}>{children}</Container>;
}
