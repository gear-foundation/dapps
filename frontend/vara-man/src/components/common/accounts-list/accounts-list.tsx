import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types'
import { decodeAddress } from '@gear-js/api'
import { Button } from '@gear-js/ui'
import { useAccount, useAlert } from '@gear-js/react-hooks'
import { copyToClipboard, isLoggedIn } from '@/app/utils'
import { LOCAL_STORAGE } from '@/app/consts'
import { AccountButton } from '@/components/common/account-button'
import { SpriteIcon } from '@/components/ui/sprite-icon'

type Props = {
  list: InjectedAccountWithMeta
  onChange: () => void
}

export const AccountsList = ({ list, onChange }: Props) => {
  const alert = useAlert()
  const { logout, login } = useAccount()

  const onClick = async (account: InjectedAccountWithMeta) => {
    await logout()
    await login(account)
    localStorage.setItem(LOCAL_STORAGE.ACCOUNT, account.address)
    onChange()
  }

  const onCopy = async (address: string) => {
    const decodedAddress = decodeAddress(address)
    await copyToClipboard({ key: decodedAddress, alert })
  }
  
  return list ? (
    <>
      <AccountButton
        address={list.address}
        name={list.meta.name}
        isActive={isLoggedIn(list)}
        onClick={() => onClick(list)}
      />
      <Button
        icon={() => <SpriteIcon name="copy" className="w-5 h-5" />}
        color="transparent"
        onClick={() => onCopy(list.address)}
      />
    </>
  ) : (
    <p>
      No accounts found. Please open Polkadot extension, create a new account or
      import existing one and reload the page.
    </p>
  )
}
