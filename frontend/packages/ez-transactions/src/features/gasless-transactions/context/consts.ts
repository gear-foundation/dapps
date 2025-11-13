import { HexString } from '@gear-js/api';

export const DEFAULT_GASLESS_CONTEXT = {
  voucherId: undefined,
  isLoading: false,
  isEnabled: false,
  isActive: false,
  voucherStatus: null,
  expireTimestamp: null,
  requestVoucher: () => Promise.resolve<HexString>('0x'),
  setIsEnabled: () => {},
};
