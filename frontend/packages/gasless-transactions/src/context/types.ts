import { HexString } from '@gear-js/api';

export type GaslessContext = {
  voucherId: HexString | undefined;
  isAvailable: boolean;
  isLoading: boolean;
  isEnabled: boolean;
  isActive: boolean;
  requestVoucher: (accountAddress: string) => Promise<void>;
  setIsEnabled: (value: boolean) => void;
};
