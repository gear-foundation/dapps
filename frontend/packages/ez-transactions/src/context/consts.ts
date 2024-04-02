import { DEFAULT_GASLESS_CONTEXT } from '@dapps-frontend/gasless-transactions';
import { DEFAULT_SIGNLESS_CONTEXT } from '@dapps-frontend/signless-transactions';

const DEFAULT_VALUES = {
  gasless: DEFAULT_GASLESS_CONTEXT,
  signless: {
    ...DEFAULT_SIGNLESS_CONTEXT,
    onSessionCreate: async () => undefined,
  },
};

export { DEFAULT_VALUES };
