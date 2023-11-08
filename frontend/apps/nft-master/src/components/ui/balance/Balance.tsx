import clsx from 'clsx'
import { PropsWithChildren } from 'react'
import { TVaraCoinIcon, VaraCoinIcon } from 'assets/images'
import { useAccountAvailableBalance } from 'features/available-balance/hooks'
import styles from './Balance.module.scss'
import { SVGComponent } from '../../../types'

type Props = PropsWithChildren & {
  className?: string
  SVG: SVGComponent
  value: string
  decimal?: string
  unit?: string
}

type HOCProps = Omit<Props, 'SVG' | 'children'>

function Balance({ SVG, value, decimal, unit, className }: Props) {
  return (
    <span className={clsx(styles.wrapper, className)}>
      <SVG />
      <span className={styles.balance}>
        <b
          className={styles.amount}
          dangerouslySetInnerHTML={{ __html: value }}
        />
        {decimal && (
          <span className={clsx(styles.small, styles.decimal)}>.{decimal}</span>
        )}
        {unit && (
          <span className={clsx(styles.small, styles.unit)}>{unit}</span>
        )}
      </span>
    </span>
  )
}

export function VaraBalance({ value, unit, className }: HOCProps) {
  const v = value.split('.')

  return (
    <Balance
      SVG={unit?.toLowerCase() === 'vara' ? VaraCoinIcon : TVaraCoinIcon}
      value={v[0].replaceAll(/,|\s/g, '&thinsp;')}
      decimal={v[1]}
      unit={unit}
      className={className}
    />
  )
}

export function AccountBalance({ className }: Pick<HOCProps, 'className'>) {
  const { availableBalance: balance } = useAccountAvailableBalance()

  return (
    <VaraBalance
      value={balance?.value || '0'}
      unit={balance?.unit || 'VARA'}
      className={className}
    />
  )
}

// function PointsBalance({ value, unit = 'PPV', className }: HOCProps) {
//   return <Balance SVG={BonusPointsIcon} value={value} unit={unit} className={className} />;
// }
