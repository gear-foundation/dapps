import styles from './Copyright.module.scss';
import { Text } from '../../../ui/text';

function Copyright() {
  const year = new Date().getFullYear();

  return (
    <Text size="sm" className={styles.copyright}>
      &copy; {year} Gear Foundation, Inc. All Rights Reserved.
    </Text>
  );
}

export { Copyright };
