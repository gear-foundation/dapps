import { HexString } from '@gear-js/api';

export type GaslessContext = {
  voucherId: HexString | undefined;
  isAvailable: boolean;
  isLoading: boolean;
  isEnabled: boolean;
  requestVoucher: (signlessAccountAddress?: string) => Promise<void>;
  setIsEnabled: (value: boolean) => void;
};
