import { GaslessContext } from '@dapps-frontend/gasless-transactions';
import { SignlessContext } from '@dapps-frontend/signless-transactions';

type Value = {
  gasless: GaslessContext;
  signless: SignlessContext & {
    onSessionCreate: (signlessAccountAddress: string) => Promise<`0x${string}` | undefined>;
  };
};

export type { Value };
