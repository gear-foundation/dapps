import { initGasslessTransactions } from '@dapps-frontend/gasless-transactions';
import { BATTLE_ADDRESS } from '../consts';
import { ENV } from 'app/consts';

export const { useFetchVoucher } = initGasslessTransactions({
  programId: BATTLE_ADDRESS,
  backendAddress: ENV.BACK,
});
