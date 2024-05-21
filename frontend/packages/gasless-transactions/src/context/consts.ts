export const DEFAULT_GASLESS_CONTEXT = {
  voucherId: undefined,
  isLoading: false,
  isEnabled: false,
  isActive: false,
  voucherStatus: null,
  requestVoucher: async (): Promise<`0x${string}`> => '0x',
  setIsEnabled: () => {},
};
