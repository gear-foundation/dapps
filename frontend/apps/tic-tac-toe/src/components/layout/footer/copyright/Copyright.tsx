import styles from './Copyright.module.scss';
import { Text } from '@/components/ui/text';
import clsx from 'clsx';

export function Copyright({ className }: { className?: string }) {
  const year = new Date().getFullYear();

  return (
    <Text size="sm" className={clsx(styles.copyright, className)}>
      &copy; {year} Gear Foundation, Inc. All Rights Reserved.
    </Text>
  );
}
