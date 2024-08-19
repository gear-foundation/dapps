import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider as GearAccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import {
  SignlessTransactionsProvider as SharedSignlessTransactionsProvider,
  GaslessTransactionsProvider as SharedGaslessTransactionsProvider,
  EzTransactionsProvider,
} from '@dapps-frontend/ez-transactions';

import { ADDRESS } from '@/app/consts';
import { Alert, alertStyles } from '@/components/ui/alert';
import { QueryProvider } from './query-provider';

function ApiProvider({ children }: ProviderProps) {
  return <GearApiProvider initialArgs={{ endpoint: ADDRESS.NODE }}>{children}</GearApiProvider>;
}

function AccountProvider({ children }: ProviderProps) {
  return <GearAccountProvider appName="Vara ZK Battleship">{children}</GearAccountProvider>;
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
    <SharedGaslessTransactionsProvider
      programId={ADDRESS.GAME}
      backendAddress={ADDRESS.GASLESS_BACKEND}
      voucherLimit={18}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

// function SignlessTransactionsProvider({ children }: ProviderProps) {
//   return (
//     <SharedSignlessTransactionsProvider programId={ADDRESS.GAME} metadataSource={metaTxt}>
//       {children}
//     </SharedSignlessTransactionsProvider>
//   );
// }

const providers = [
  BrowserRouter,
  ApiProvider,
  AccountProvider,
  AlertProvider,
  QueryProvider,
  GaslessTransactionsProvider,
  // SignlessTransactionsProvider,
  EzTransactionsProvider,
];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
