import clsx from 'clsx'

import { useAccount } from '@gear-js/react-hooks'

import { VaraBalance } from '@/components/ui/balance'
import { Button } from '@/components/ui/button'

import { AvaVaraBlack, ChevronDown, CrossIcon } from '@/assets/images'

import styles from './account-info.module.scss'

type AccountInfoProps = BaseComponentProps & {
  openWallet: () => void
  isOpen: boolean
}

export function AccountInfo({ className, openWallet, isOpen }: AccountInfoProps) {
  const { account } = useAccount()

  return (
    <>
      <div className={clsx(styles.wrapper, className)}>
        {!!account && (
          <>
            <VaraBalance
              value={account.balance.value}
              unit={account.balance.unit}
              className={styles.balance}
            />
            <Button variant="text" className={styles.openWallet} onClick={openWallet}>

              {isOpen ? <CrossIcon /> :
                <>
                  <AvaVaraBlack width={24} height={24} />
                  <ChevronDown />
                </>
              }
            </Button>
          </>
        )}
      </div>
    </>
  )
}
