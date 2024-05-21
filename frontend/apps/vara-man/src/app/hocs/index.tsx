import type { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';
import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import {
  SignlessTransactionsProvider as SharedSignlessTransactionsProvider,
  GaslessTransactionsProvider as SharedGaslessTransactionsProvider,
  EzTransactionsProvider,
} from '@dapps-frontend/ez-transactions';
import { ENV } from '@/app/consts';
import { AppProvider } from '@/app/context/ctx-app';
import { GameProvider } from '@/app/context/ctx-game';
import { Alert, alertStyles } from '@/components/ui/alert';
import metaTxt from '@/assets/meta/vara_man.meta.txt';

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>
);

const AlertProvider = ({ children }: ProviderProps) => (
  <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
    {children}
  </GearAlertProvider>
);

const BrowserRouterProvider = ({ children }: ProviderProps) => <BrowserRouter>{children}</BrowserRouter>;

function GaslessTransactionsProvider({ children }: ProviderProps) {
  return (
    <SharedGaslessTransactionsProvider programId={ENV.GAME} backendAddress={ENV.BACK} voucherLimit={18}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

function SignlessTransactionsProvider({ children }: ProviderProps) {
  return (
    <SharedSignlessTransactionsProvider programId={ENV.GAME} metadataSource={metaTxt}>
      {children}
    </SharedSignlessTransactionsProvider>
  );
}

const providers = [
  BrowserRouterProvider,
  AlertProvider,
  ApiProvider,
  AccountProvider,
  AppProvider,
  GameProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
  EzTransactionsProvider,
];

export const withProviders = (Component: ComponentType) => () =>
  providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
