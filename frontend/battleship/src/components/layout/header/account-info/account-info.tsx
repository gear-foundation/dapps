import clsx from 'clsx'

import {
  useApi,
  useAccount,
  useBalance,
  useBalanceFormat,
} from '@gear-js/react-hooks'

import { VaraBalance } from '@/components/ui/balance'
import { Button } from '@/components/ui/button'
import { AvaVaraBlack, ChevronDown, CrossIcon } from '@/assets/images'

import styles from './account-info.module.scss'

type AccountInfoProps = BaseComponentProps & {
  openWallet: () => void
  isOpen: boolean
}

export function AccountInfo({
  className,
  openWallet,
  isOpen,
}: AccountInfoProps) {
  const { isApiReady } = useApi()
  const { account } = useAccount()
  const { balance } = useBalance(account?.address)
  const { getFormattedBalance } = useBalanceFormat()
  const formattedBalance =
    isApiReady && balance ? getFormattedBalance(balance) : undefined

  return (
    <>
      <div className={clsx(styles.wrapper, className)}>
        {!!account && (
          <>
            {formattedBalance && (
              <VaraBalance
                value={formattedBalance.value}
                unit={formattedBalance.unit}
                className={styles.balance}
              />
            )}

            <Button
              variant="text"
              className={styles.openWallet}
              onClick={openWallet}
            >
              {isOpen ? (
                <CrossIcon />
              ) : (
                <>
                  <AvaVaraBlack width={24} height={24} />
                  <ChevronDown />
                </>
              )}
            </Button>
          </>
        )}
      </div>
    </>
  )
}
