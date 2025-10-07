import { GaslessContext } from '@ez/features/gasless-transactions';
import { SignlessContext } from '@ez/features/signless-transactions';

type Value = {
  gasless: GaslessContext;
  signless: SignlessContext & {
    onSessionCreate: (signlessAccountAddress: string) => Promise<`0x${string}`>;
  };
  isAutoSignlessEnabled: boolean;
};

export type { Value };
