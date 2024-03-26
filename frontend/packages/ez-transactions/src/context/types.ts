import { GaslessContext } from '@dapps-frontend/gasless-transactions';
import { SignlessContext } from '@dapps-frontend/signless-transactions';

type Value = {
  gasless: GaslessContext;
  signless: SignlessContext & {
    isActive: boolean;
    onSessionCreate: (signlessAccountAddress: string) => Promise<void>;
  };
};

export type { Value };
