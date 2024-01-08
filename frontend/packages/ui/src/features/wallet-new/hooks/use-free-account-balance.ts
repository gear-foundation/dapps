import { useAccount, useApi } from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';

function useFreeAccountBalance() {
  const { api, isApiReady } = useApi();
  const { account } = useAccount();
  const { address } = account || {};

  const [freeAccountBalance, setFreeAccountBalance] = useState('');

  useEffect(() => {
    if (!isApiReady || !address) return;

    const unsub = api.derive.balances.all(address, (result) => setFreeAccountBalance(result.freeBalance.toString()));

    return () => {
      setFreeAccountBalance('');
      unsub.then((unsubCallback) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, address]);

  return { freeAccountBalance };
}

export { useFreeAccountBalance };
