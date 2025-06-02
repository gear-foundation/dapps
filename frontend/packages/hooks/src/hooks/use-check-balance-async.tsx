import { useAccount, useApi, useBalance, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { stringShorten } from '@polkadot/util';

type Props = {
  gaslessVoucherId?: `0x${string}`;
  signlessPairVoucherId?: string;
};

function useCheckBalanceAsync(args?: Props) {
  const { signlessPairVoucherId, gaslessVoucherId } = args || {};
  const { api } = useApi();
  const { account } = useAccount();
  const voucherAddress = signlessPairVoucherId || account?.decodedAddress;

  const { balance } = useBalance(gaslessVoucherId || voucherAddress);
  const { getFormattedBalanceValue, getFormattedGasValue } = useBalanceFormat();

  const checkBalance = (limit: number) =>
    new Promise((resolve, reject) => {
      const chainBalance = Number(getFormattedBalanceValue(Number(withoutCommas(balance?.toString() || ''))).toFixed());
      const valuePerGas = Number(withoutCommas(getFormattedGasValue(api!.valuePerGas.toHuman()).toFixed()));
      const chainEDeposit = Number(
        getFormattedBalanceValue(Number(withoutCommas(api?.existentialDeposit.toString() || ''))).toFixed(),
      );
      const gasLimit = Number(getFormattedGasValue(limit).toFixed());

      const chainEDepositWithLimit = chainEDeposit + gasLimit * valuePerGas;

      if (chainBalance < chainEDepositWithLimit) {
        const errorText = `Low balance on ${stringShorten(account?.decodedAddress || '', 8)}`;
        reject(new Error(errorText));
      }

      resolve(true);
    });

  return { checkBalance };
}

export { useCheckBalanceAsync };
