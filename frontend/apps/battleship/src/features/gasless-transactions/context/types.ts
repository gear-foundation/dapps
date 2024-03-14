import { HexString } from '@gear-js/api';

export type Value = {
  voucherId: HexString | undefined;
  isAvailable: boolean;
  isLoading: boolean;
  isEnabled: boolean;
  requestVoucher: () => Promise<void>;
  setIsEnabled: (value: boolean) => void;
};
