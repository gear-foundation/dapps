import { Button, buttonStyles } from '@gear-js/ui'
import { lazy } from 'react'
import { cn } from '@/app/utils'

const Identicon = lazy(() => import('@polkadot/react-identicon'))

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
      'w-full !justify-start',
      isActive ? buttonStyles.primary : buttonStyles.light
    )}
    text={name}
    onClick={onClick}
    icon={() => (
      <Identicon
        value={address}
        className={buttonStyles.icon}
        theme="polkadot"
        size={28}
      />
    )}
  />
)
