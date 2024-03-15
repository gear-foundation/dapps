import { DEFAULT_VALUES as GASLESS_DEFAULT_VALUES } from '@/features/gasless-transactions/context/consts';
import { DEFAULT_VALUES as SIGNLESS_DEFAULT_VALUES } from '@/features/signless-transactions/context/consts';

const DEFAULT_VALUES = {
  gasless: GASLESS_DEFAULT_VALUES,
  signless: {
    ...SIGNLESS_DEFAULT_VALUES,
    isActive: false,
    onSessionCreate: () => {},
  },
};

export { DEFAULT_VALUES };
