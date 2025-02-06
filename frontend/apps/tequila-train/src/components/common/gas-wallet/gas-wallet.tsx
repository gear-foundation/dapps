import { useApi, useBalance, useBalanceFormat } from '@gear-js/react-hooks';

import { GetGasBalance } from '@/components/common/get-gas-balance';

type Props = {
  address: string;
  name: string | undefined;
  onClick: () => void;
};

export function GasWallet({ address }: Props) {
  const { isApiReady } = useApi();
  const { balance } = useBalance(address);
  const { getFormattedBalance } = useBalanceFormat();
  const formattedBalance = isApiReady && balance ? getFormattedBalance(balance) : undefined;

  return formattedBalance ? (
    <div className="flex gap-4 shrink-0">
      <GetGasBalance />
      <p className="shrink-0 grid grid-cols-[auto_auto] gap-x-1 font-kanit">
        <span className="col-span-2 text-[10px] text-dark-400">Gas Balance:</span>
        <span className="font-medium text-lg leading-none text-dark-500">{formattedBalance?.value}</span>
        <span className="text-sm text-dark-400">{formattedBalance?.unit}</span>
      </p>
    </div>
  ) : null;
}
