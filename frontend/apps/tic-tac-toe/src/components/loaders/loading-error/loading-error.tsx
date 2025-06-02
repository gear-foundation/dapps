import { BaseComponentProps } from '@/app/types';
import { Container } from '@/components/ui/container';

import styles from './loading-error.module.scss';

type LoadingErrorProps = BaseComponentProps & {};

export function LoadingError({ children }: LoadingErrorProps) {
  return <Container className={styles.box}>{children}</Container>;
}
