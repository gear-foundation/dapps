export const DEFAULT_GASLESS_CONTEXT = {
  voucherId: undefined,
  isAvailable: false,
  isLoading: false,
  isEnabled: false,
  isActive: false,
  requestVoucher: async (): Promise<`0x${string}`> => '0x',
  setIsEnabled: () => {},
};
