import { useAccount, useBalanceFormat, useDeriveBalancesAll } from '@gear-js/react-hooks';
import { useState, useEffect } from 'react';

function useIsAvailable(requiredBalance: number, isSessionActive: boolean) {
  const { account } = useAccount();
  const { data: balances } = useDeriveBalancesAll({ address: account?.address, watch: true });
  const { getChainBalanceValue } = useBalanceFormat();

  const [isAvailable, setIsAvailable] = useState(false);

  useEffect(() => {
    if (isSessionActive) return setIsAvailable(true);
    if (!balances) return setIsAvailable(false);

    const freeBalance = balances.freeBalance.toString();
    const result = getChainBalanceValue(requiredBalance).isLessThanOrEqualTo(freeBalance);

    setIsAvailable(result);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [balances, requiredBalance, isSessionActive]);

  return isAvailable;
}

export { useIsAvailable };
