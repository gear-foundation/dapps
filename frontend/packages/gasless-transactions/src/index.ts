import { useFetchVoucher as useFetchVoucherHook } from './hooks';
import { InitGasslessTransactions } from './types';

export const initGasslessTransactions = ({ programId, backendAddress, voucherLimit }: InitGasslessTransactions) => ({
  useFetchVoucher: () => useFetchVoucherHook({ programId, backendAddress, voucherLimit }),
});
