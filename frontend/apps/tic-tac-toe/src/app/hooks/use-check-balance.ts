import { useAccountAvailableBalance } from '@/features/account-available-balance/hooks';
import { useAccount, useAlert, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { stringShorten } from '@polkadot/util';
import { withoutCommas } from '../utils';

export function useCheckBalance() {
  const { api } = useApi();
  const { account } = useAccount();
  const { availableBalance } = useAccountAvailableBalance();
  const { getChainBalanceValue } = useBalanceFormat();
  const alert = useAlert();

  const checkBalance = (limit: number, callback: () => void, onError?: () => void) => {
    const chainBalance = Number(getChainBalanceValue(Number(withoutCommas(availableBalance!.value))).toFixed());
    const valuePerGas = Number(withoutCommas(api!.valuePerGas!.toHuman()));

    const chainEDeposit = Number(
      getChainBalanceValue(Number(withoutCommas(availableBalance?.existentialDeposit || ''))).toFixed(),
    );

    const chainEDepositWithLimit = chainEDeposit + limit * valuePerGas;
    console.log('LIMIT:');
    console.log(limit);
    console.log(limit * valuePerGas);
    console.log('existentialDeposit:');
    console.log(Number(withoutCommas(availableBalance?.existentialDeposit || '')));
    console.log('eDeposit');
    console.log(chainEDeposit);
    console.log('eDeposit + Limit:');
    console.log(chainEDepositWithLimit);
    console.log('balance:');
    console.log(Number(withoutCommas(availableBalance!.value)));
    console.log('chain balance:');
    console.log(chainBalance);
    console.log('low balance: ');
    console.log(chainBalance < chainEDepositWithLimit);

    if (!chainBalance || chainBalance < chainEDepositWithLimit) {
      alert.error(`Low balance on ${stringShorten(account?.decodedAddress || '', 8)}`);

      if (onError) {
        onError();
      }

      return;
    }

    callback();
  };

  return { checkBalance };
}
