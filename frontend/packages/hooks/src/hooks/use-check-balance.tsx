import {
  useAccount,
  useAlert,
  useApi,
  useBalance,
  useBalanceFormat,
  useVouchers,
  withoutCommas,
} from '@gear-js/react-hooks';
import { HexString, decodeAddress } from '@gear-js/api';
import { stringShorten } from '@polkadot/util';
import { KeyringPair } from '@polkadot/keyring/types';

function useCheckBalance(programId: HexString, signlessPair?: KeyringPair) {
  const { api } = useApi();
  const { account } = useAccount();
  const { vouchers, isEachVoucherReady } = useVouchers(account?.decodedAddress, programId);
  const voucherAddress = signlessPair ? decodeAddress(signlessPair.address) : account?.decodedAddress;
  const voucherKeys = isEachVoucherReady && vouchers ? Object.keys(vouchers) : [];
  const firstVoucherKey = voucherKeys[0] as `0x${string}`;
  const { balance } = useBalance(vouchers && voucherKeys.length ? vouchers[firstVoucherKey].owner : voucherAddress);
  const { getFormattedBalanceValue, getFormattedGasValue } = useBalanceFormat();
  const alert = useAlert();

  const checkBalance = (limit: number, callback: () => void, onError?: () => void) => {
    const chainBalance = Number(getFormattedBalanceValue(Number(withoutCommas(balance?.toString() || ''))).toFixed());
    const valuePerGas = Number(withoutCommas(api!.valuePerGas!.toHuman()));
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
