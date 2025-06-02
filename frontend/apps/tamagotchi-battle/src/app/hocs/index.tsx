import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider as GearAccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { Alert, alertStyles } from '@gear-js/ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { GaslessTransactionsProvider as SharedGaslessTransactionsProvider } from 'gear-ez-transactions';
import type { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { DnsProvider as SharedDnsProvider, useDnsProgramIds } from '@dapps-frontend/hooks';

import { ENV } from '@/app/consts';
import { BattleProvider } from '@/features/battle/context';

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>
);

function AccountProvider({ children }: ProviderProps) {
  return <GearAccountProvider appName="Vara Tamagotchi Battle">{children}</GearAccountProvider>;
}

function DnsProvider({ children }: ProviderProps) {
  return (
    <SharedDnsProvider names={{ programId: ENV.DNS_NAME }} dnsApiUrl={ENV.DNS_API_URL}>
      {children}
    </SharedDnsProvider>
  );
}

function GaslessTransactionsProvider({ children }: ProviderProps) {
  const { programId } = useDnsProgramIds();
  return (
    <SharedGaslessTransactionsProvider programId={programId} backendAddress={ENV.GASLESS_BACKEND} voucherLimit={18}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

const AlertProvider = ({ children }: ProviderProps) => (
  <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
    {children}
  </GearAlertProvider>
);

const queryClient = new QueryClient();

function QueryProvider({ children }: ProviderProps) {
  return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>;
}

const providers = [
  BrowserRouter,
  AlertProvider,
  ApiProvider,
  QueryProvider,
  DnsProvider,
  AccountProvider,
  BattleProvider,
  GaslessTransactionsProvider,
];

export const withProviders = (Component: ComponentType) => () =>
  providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
