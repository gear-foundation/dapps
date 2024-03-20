import { DEFAULT_GASLESS_CONTEXT } from '@/features/gasless-transactions';
import { DEFAULT_SIGNLESS_CONTEXT } from '@/features/signless-transactions';

const DEFAULT_VALUES = {
  gasless: DEFAULT_GASLESS_CONTEXT,
  signless: {
    ...DEFAULT_SIGNLESS_CONTEXT,
    isActive: false,
    onSessionCreate: async () => {},
  },
};

export { DEFAULT_VALUES };
