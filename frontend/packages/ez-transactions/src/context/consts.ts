import { HexString } from '@gear-js/api';

import { DEFAULT_GASLESS_CONTEXT } from '@ez/features/gasless-transactions';
import { DEFAULT_SIGNLESS_CONTEXT } from '@ez/features/signless-transactions';

const DEFAULT_VALUES = {
  gasless: DEFAULT_GASLESS_CONTEXT,
  signless: {
    ...DEFAULT_SIGNLESS_CONTEXT,
    onSessionCreate: () => Promise.resolve<HexString>('0x'),
  },
};

export { DEFAULT_VALUES };
