import { HexString } from '@gear-js/api';

export type Value = {
  voucherId: HexString | undefined;
  isLoadingVoucher: boolean;
  isAvailable: boolean;
  isLoading: boolean;
  isActive: boolean;
  setIsActive: React.Dispatch<React.SetStateAction<boolean>>;
};
