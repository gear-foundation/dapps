import clsx from 'clsx'
import styles from './account-info.module.scss'
import { Wallet } from '@/features/wallet'
import { Account, useAccount } from '@gear-js/react-hooks'
import { useFTBalance } from '@/features/points-balance/hooks'
import { PointsBalance, VaraBalance } from '@/components/ui/balance'
import { useInitGame } from '@/features/tic-tac-toe/hooks'

type AccountInfoProps = BaseComponentProps & {}

function AccountPointsBalance() {
  const { balance, isFTBalanceReady } = useFTBalance()

  return isFTBalanceReady ? <PointsBalance value={balance} /> : null
}

function AccountVaraBalance({ account }: { account: Account }) {
  return (
    <VaraBalance
      value={account.balance.value}
      unit={account.balance.unit}
      className={styles.vara}
    />
  )
}

export function AccountInfo({ className }: AccountInfoProps) {
  const { account } = useAccount()
  const { isGameReady } = useInitGame()
  const { isFTBalanceReady } = useFTBalance()
  const isUserReady = isGameReady && isFTBalanceReady

  return (
    <div className={clsx(styles.wrapper, className)}>
      {isUserReady && !!account && (
        <>
          <AccountPointsBalance />
          <AccountVaraBalance account={account} />
        </>
      )}

      <Wallet account={account} isReady={isUserReady} />
    </div>
  )
}
