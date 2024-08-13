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
import { DnsProvider as SharedDnsProvider, useDnsProgramIds } from '@dapps-frontend/hooks';
import { BattleProvider } from 'features/battle/context';
import { ENV } from 'app/consts';

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>
);

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

const providers = [
  BrowserRouter,
  AlertProvider,
  ApiProvider,
  DnsProvider,
  AccountProvider,
  BattleProvider,
  GaslessTransactionsProvider,
];

export const withProviders = (Component: ComponentType) => () =>
  providers.reduceRight(
    (children, Provider) => <Provider appName="Vara Tamagotchi Battle">{children}</Provider>,
    <Component />,
  );
