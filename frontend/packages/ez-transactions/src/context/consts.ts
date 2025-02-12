import { DEFAULT_GASLESS_CONTEXT } from '@ez/features/gasless-transactions';
import { DEFAULT_SIGNLESS_CONTEXT } from '@ez/features/signless-transactions';

const DEFAULT_VALUES = {
  gasless: DEFAULT_GASLESS_CONTEXT,
  signless: {
    ...DEFAULT_SIGNLESS_CONTEXT,
    onSessionCreate: async (): Promise<`0x${string}`> => '0x',
  },
};

export { DEFAULT_VALUES };
