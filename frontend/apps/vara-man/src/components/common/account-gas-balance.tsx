import {
  useAccount,
  useApi,
  useBalance,
  useBalanceFormat,
} from '@gear-js/react-hooks'

export function AccountGasBalance() {
  const { isApiReady } = useApi()
  const { account } = useAccount()
  const { balance } = useBalance(account?.address)
  const { getFormattedBalance } = useBalanceFormat()
  const formattedBalance =
    isApiReady && balance ? getFormattedBalance(balance) : undefined

  return formattedBalance ? (
    <div className="flex space-x-4 shrink-0">
      <p className="shrink-0 grid grid-cols-[auto_auto] gap-x-1 font-kanit">
        <span className="col-span-2 text-[10px] text-dark-400">
          Gas Balance:
        </span>
        <span className="font-medium text-lg leading-none ">
          {formattedBalance.value}
        </span>
        <span className="text-sm ">{formattedBalance.unit}</span>
      </p>
    </div>
  ) : null
}
