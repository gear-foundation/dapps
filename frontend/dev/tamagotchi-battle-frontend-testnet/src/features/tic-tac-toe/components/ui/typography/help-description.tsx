import styles from './typography.module.scss'
import clsx from 'clsx'
import { textVariants } from '@/components/ui/text/text'

type HelpDescriptionProps = React.PropsWithChildren & {}

export function HelpDescription({ children }: HelpDescriptionProps) {
  return (
    <div className={clsx(styles.description, textVariants({ size: 'lg' }))}>
      {children}
    </div>
  )
}
