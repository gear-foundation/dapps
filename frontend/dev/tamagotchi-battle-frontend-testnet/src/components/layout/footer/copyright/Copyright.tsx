import styles from './Copyright.module.scss'
import { Text } from '@/components/ui/text'

export function Copyright() {
  const year = new Date().getFullYear()

  return (
    <Text size="xs" className={styles.copyright}>
      &copy; {year} Gear Foundation, Inc. All Rights Reserved.
    </Text>
  )
}
