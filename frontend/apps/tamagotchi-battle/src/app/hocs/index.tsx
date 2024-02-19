import type { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';
import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { Alert, alertStyles } from '@gear-js/ui';
import { GaslessTransactionsProvider as SharedGaslessTransactionsProvider } from '@dapps-frontend/gasless-transactions';
import { BattleProvider } from 'features/battle/context';
import { ENV } from 'app/consts';
import { BATTLE_ADDRESS } from 'features/battle/consts';

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>
);

function GaslessTransactionsProvider({ children }: ProviderProps) {
  return (
    <SharedGaslessTransactionsProvider programId={BATTLE_ADDRESS} backendAddress={ENV.BACK} voucherLimit={18}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

const AlertProvider = ({ children }: ProviderProps) => (
  <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
    {children}
  </GearAlertProvider>
);

const providers = [
  BrowserRouter,
  AlertProvider,
  ApiProvider,
  AccountProvider,
  BattleProvider,
  GaslessTransactionsProvider,
];

export const withProviders = (Component: ComponentType) => () =>
  providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
