import { Value as GaslessValue } from '@/features/gasless-transactions/context/types';
import { Value as SignlessValue } from '@/features/signless-transactions/context/types';

type Value = {
  gasless: GaslessValue;
  signless: SignlessValue & {
    isActive: boolean;
  };
};

export type { Value };
