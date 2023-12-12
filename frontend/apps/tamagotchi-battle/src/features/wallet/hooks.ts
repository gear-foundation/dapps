import {
  useAccount,
  useAccountVoucherBalance,
  useAlert,
  useApi,
  useBalance,
  useBalanceFormat,
  withoutCommas,
} from '@gear-js/react-hooks';
import { stringShorten } from '@polkadot/util';
import { BATTLE_ADDRESS } from 'features/battle/consts';
import { VOUCHER_MIN_LIMIT } from 'app/consts';

export function useCheckBalance(isVoucher: boolean) {
  const { api } = useApi();
  const { account } = useAccount();
  const { balance: availableBalance } = useBalance(account?.decodedAddress);
  const { getChainBalanceValue, getFormattedBalanceValue } = useBalanceFormat();
  const { voucherBalance } = useAccountVoucherBalance(BATTLE_ADDRESS);
  const alert = useAlert();

  const checkBalance = (limit: number, callback: () => void, onError?: () => void) => {
    const chainBalance = Number(
      getChainBalanceValue(Number(withoutCommas(availableBalance?.toString() || ''))).toFixed(),
    );
    const valuePerGas = Number(withoutCommas(api!.valuePerGas!.toHuman()));
    const chainEDeposit = Number(
      getChainBalanceValue(Number(withoutCommas(api?.existentialDeposit.toString() || ''))).toFixed(),
    );

    const chainEDepositWithLimit = chainEDeposit + limit * valuePerGas;

    if (
      isVoucher && !!voucherBalance
        ? getFormattedBalanceValue(voucherBalance.toString()).toFixed() < VOUCHER_MIN_LIMIT
        : !chainBalance || chainBalance < chainEDepositWithLimit
    ) {
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
