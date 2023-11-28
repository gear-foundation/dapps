import clsx from 'clsx'
import styles from '../columns.module.scss'

type ContainerProps = BaseComponentProps & {}

export function ColumnRight({ children, className }: ContainerProps) {
  return (
    <div className={clsx(styles.container__right, className)}>{children}</div>
  )
}
