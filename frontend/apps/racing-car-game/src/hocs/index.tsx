import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider as GearAccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import {
  SignlessTransactionsProvider as SharedSignlessTransactionsProvider,
  GaslessTransactionsProvider as SharedGaslessTransactionsProvider,
  EzTransactionsProvider,
} from 'gear-ez-transactions';
import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { DnsProvider as SharedDnsProvider, useDnsProgramIds } from '@dapps-frontend/hooks';
import { QueryProvider } from '@dapps-frontend/ui';

import { useProgram } from '@/app/utils';
import { ENV } from '@/consts';
import { Alert, alertStyles } from '@/ui';

function DnsProvider({ children }: ProviderProps) {
  return (
    <SharedDnsProvider names={{ programId: ENV.DNS_NAME }} dnsApiUrl={ENV.DNS_API_URL}>
      {children}
    </SharedDnsProvider>
  );
}

function ApiProvider({ children }: ProviderProps) {
  return <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>;
}

function AccountProvider({ children }: ProviderProps) {
  return <GearAccountProvider appName="Vara Racing Cars">{children}</GearAccountProvider>;
}

function AlertProvider({ children }: ProviderProps) {
  return (
    <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
      {children}
    </GearAlertProvider>
  );
}

function GaslessTransactionsProvider({ children }: ProviderProps) {
  const { programId } = useDnsProgramIds();

  return (
    <SharedGaslessTransactionsProvider
      programId={programId}
      backendAddress={ENV.GASLESS_BACKEND}
      voucherLimit={Number(ENV.VOUCHER_LIMIT)}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

function SignlessTransactionsProvider({ children }: ProviderProps) {
  const { programId } = useDnsProgramIds();
  const program = useProgram();
  return (
    <SharedSignlessTransactionsProvider programId={programId} program={program}>
      {children}
    </SharedSignlessTransactionsProvider>
  );
}

const providers = [
  BrowserRouter,
  AlertProvider,
  ApiProvider,
  DnsProvider,
  QueryProvider,
  AccountProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
  EzTransactionsProvider,
];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
