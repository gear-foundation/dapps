import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { DnsProvider as SharedDnsProvider, useDnsProgramId } from '@dapps-frontend/hooks';
import {
  SignlessTransactionsProvider as SharedSignlessTransactionsProvider,
  GaslessTransactionsProvider as SharedGaslessTransactionsProvider,
  EzTransactionsProvider,
} from '@dapps-frontend/ez-transactions';

import metaTxt from '@/features/tic-tac-toe/assets/meta/tic_tac_toe.meta.txt';
import { ADDRESS } from '@/app/consts';
import { Alert, alertStyles } from '@/components/ui/alert';

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

function DnsProvider({ children }: ProviderProps) {
  return (
    <SharedDnsProvider name={ADDRESS.DNS_NAME} dnsApiUrl={ADDRESS.DNS_API_URL}>
      {children}
    </SharedDnsProvider>
  );
}

function GaslessTransactionsProvider({ children }: ProviderProps) {
  const programId = useDnsProgramId();
  return (
    <SharedGaslessTransactionsProvider programId={programId} backendAddress={ADDRESS.GASLESS_BACKEND} voucherLimit={6}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

function SignlessTransactionsProvider({ children }: ProviderProps) {
  const programId = useDnsProgramId();
  return (
    <SharedSignlessTransactionsProvider programId={programId} metadataSource={metaTxt}>
      {children}
    </SharedSignlessTransactionsProvider>
  );
}

const providers = [
  BrowserRouter,
  ApiProvider,
  AccountProvider,
  AlertProvider,
  DnsProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
  EzTransactionsProvider,
];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
