import { useFetchVoucher as useFetchVoucherHook } from './hooks';
import { InitVoucher } from './types';

export const initVoucher = ({ programId, backendAddress, voucherLimit }: InitVoucher) => ({
  useFetchVoucher: () => useFetchVoucherHook({ programId, backendAddress, voucherLimit }),
});
