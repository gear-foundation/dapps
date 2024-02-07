import { useAccount, useAlert, useApi, useBalance, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { decodeAddress } from '@gear-js/api';
import { stringShorten } from '@polkadot/util';
import { KeyringPair } from '@polkadot/keyring/types';

type Props = {
  gaslessVoucherId?: `0x${string}`;
  signlessPair?: KeyringPair;
};

function useCheckBalance(args?: Props) {
  const { signlessPair, gaslessVoucherId } = args || {};
  const { api } = useApi();
  const { account } = useAccount();
  const voucherAddress = signlessPair ? decodeAddress(signlessPair.address) : account?.decodedAddress;

  const { balance } = useBalance(gaslessVoucherId || voucherAddress);
  const { getFormattedBalanceValue, getFormattedGasValue } = useBalanceFormat();
  const alert = useAlert();

  const checkBalance = (limit: number, callback: () => void, onError?: () => void) => {
    const chainBalance = Number(getFormattedBalanceValue(Number(withoutCommas(balance?.toString() || ''))).toFixed());
    const valuePerGas = Number(withoutCommas(getFormattedGasValue(api!.valuePerGas!.toHuman()).toFixed()));
    const chainEDeposit = Number(
      getFormattedBalanceValue(Number(withoutCommas(api?.existentialDeposit.toString() || ''))).toFixed(),
    );
    const gasLimit = Number(getFormattedGasValue(limit).toFixed());

    const chainEDepositWithLimit = chainEDeposit + gasLimit * valuePerGas;

    if (chainBalance < chainEDepositWithLimit) {
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

export { useCheckBalance };
