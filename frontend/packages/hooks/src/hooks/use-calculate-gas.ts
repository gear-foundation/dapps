import { GasInfo, HexString, ProgramMetadata } from '@gear-js/api';
import {
  useAlert,
  useHandleCalculateGas as useCalculateGasNative,
  withoutCommas,
  useDeriveBalancesAll,
  useAccount,
  useApi,
} from '@gear-js/react-hooks';
import { AnyJson, AnyNumber } from '@polkadot/types/types';

const useHandleCalculateGas = (address: HexString, meta: ProgramMetadata | undefined) => {
  const { api } = useApi();
  const { account } = useAccount();
  const { data: balances } = useDeriveBalancesAll({ address: account?.decodedAddress, watch: true });
  const calculateGasNative = useCalculateGasNative(address, meta);

  const alert = useAlert();

  return (initPayload: AnyJson, value?: AnyNumber | undefined): Promise<GasInfo> => {
    const balance = Number(withoutCommas(balances?.freeBalance.toString() || ''));
    const existentialDeposit = Number(withoutCommas(api?.existentialDeposit.toString() || ''));

    if (!balance || balance < existentialDeposit) {
      alert.error(`Low balance when calculating gas`);
    }

    return calculateGasNative(initPayload, value);
  };
};

export { useHandleCalculateGas };
