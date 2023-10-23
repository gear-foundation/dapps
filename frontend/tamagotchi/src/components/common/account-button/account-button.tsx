import { Button, buttonStyles } from '@gear-js/ui'
import { cn } from '@/app/utils'
import { WalletIcon } from '@/features/wallet-icon'

type Props = {
  address: string
  name: string | undefined
  onClick: () => void
  isActive?: boolean
  block?: boolean
}

export const AccountButton = ({ address, name, onClick, isActive }: Props) => (
  <Button
    className={cn(
      'w-full justify-center truncate',
      isActive ? buttonStyles.primary : buttonStyles.light
    )}
    text={name}
    onClick={onClick}
    icon={() => (
      <WalletIcon
        address={address}
        className={cn(buttonStyles.icon, 'shrink-0')}
        size={28}
      />
    )}
  />
)
