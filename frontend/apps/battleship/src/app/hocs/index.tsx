import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { SignlessTransactionsProvider as SharedSignlessTransactionsProvider } from '@/features/signless-transactions';
import { GaslessTransactionsProvider as SharedGaslessTransactionsProvider } from '@/features/gasless-transactions';

import metaTxt from '@/features/game/assets/meta/battleship.meta.txt';
import { ADDRESS } from '@/app/consts';
import { Alert, alertStyles } from '@/components/ui/alert';
import { TransactionsProvider } from '@/features/transactions';

function ApiProvider({ children }: ProviderProps) {
  return <GearApiProvider initialArgs={{ endpoint: ADDRESS.NODE }}>{children}</GearApiProvider>;
}

function AlertProvider({ children }: ProviderProps) {
  return (
    <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
      {children}
    </GearAlertProvider>
  );
}

function GaslessTransactionsProvider({ children }: ProviderProps) {
  return (
    <SharedGaslessTransactionsProvider programId={ADDRESS.GAME} backendAddress={ADDRESS.BACK} voucherLimit={18}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

function SignlessTransactionsProvider({ children }: ProviderProps) {
  return (
    <SharedSignlessTransactionsProvider programId={ADDRESS.GAME} metadataSource={metaTxt}>
      {children}
    </SharedSignlessTransactionsProvider>
  );
}

const providers = [
  BrowserRouter,
  ApiProvider,
  AccountProvider,
  AlertProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
  TransactionsProvider,
];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
