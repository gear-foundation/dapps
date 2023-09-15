import { useAccount } from '@gear-js/react-hooks'

export function AccountGasBalance() {
  const { account } = useAccount()

  return (
    <div className="flex space-x-4 shrink-0">
      <p className="shrink-0 grid grid-cols-[auto_auto] gap-x-1 font-kanit">
        <span className="col-span-2 text-[10px] text-dark-400">
          Gas Balance:
        </span>
        <span className="font-medium text-lg leading-none ">
          {account?.balance.value}
        </span>
        <span className="text-sm ">{account?.balance.unit}</span>
      </p>
    </div>
  )
}
