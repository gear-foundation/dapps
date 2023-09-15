import Identicon from '@polkadot/react-identicon'
import { buttonStyles } from '@gear-js/ui'
import { cn } from '@/app/utils'

type Props = {
  address: string
  name: string | undefined
  onClick: () => void
  isActive?: boolean
  simple?: boolean
}

export const AccountButton = ({
  address,
  name,
  onClick,
  isActive,
  simple,
}: Props) => (
  <button
    className={cn(
      'btn !inline-grid !justify-start gap-2.5 w-full px-7 whitespace-nowrap',
      simple ? 'grid-cols-[28px_1fr]' : 'grid-cols-[28px_1fr_14px]',
      isActive ? 'btn--primary' : buttonStyles.light,
      buttonStyles.button
    )}
    onClick={onClick}
  >
    <Identicon
      value={address}
      className={cn(buttonStyles.icon, 'w-7 h-7 -my-2 [&>*]:cursor-pointer')}
      theme="polkadot"
      size={28}
    />
    <span className="block truncate w-full">{name}</span>
  </button>
)
