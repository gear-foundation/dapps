export const DEFAULT_VALUES = {
  voucherId: undefined,
  isLoadingVoucher: false,
  isAvailable: false,
  isLoading: false,
  isActive: false,
  setIsActive: () => {},
  checkAndFetchVoucher: (acc: string): Promise<string> =>
    new Promise((res) => {
      res(acc);
    }),
};
