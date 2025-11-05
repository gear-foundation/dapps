import {
  ApiProvider as GearApiProvider,
  AccountProvider as GearAccountProvider,
  AlertProvider as GearAlertProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { Alert, alertStyles } from '@gear-js/ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { GaslessTransactionsProvider as SharedGaslessTransactionsProvider } from 'gear-ez-transactions';

import { DnsProvider as SharedDnsProvider, useDnsProgramIds } from '@dapps-frontend/hooks';

import { ENV } from '@/app/consts';
import { BattleProvider } from '@/features/battle/context';

const queryClient = new QueryClient();

function ApiProvider({ children }: ProviderProps) {
  return <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>;
}

function AccountProvider({ children }: ProviderProps) {
  return <GearAccountProvider appName="Vara Tamagotchi Battle">{children}</GearAccountProvider>;
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
    <SharedDnsProvider names={{ programId: ENV.DNS_NAME }} dnsApiUrl={ENV.DNS_API_URL}>
      {children}
    </SharedDnsProvider>
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

function QueryProvider({ children }: ProviderProps) {
  return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>;
}

export {
  AccountProvider,
  AlertProvider,
  ApiProvider,
  BattleProvider,
  DnsProvider,
  GaslessTransactionsProvider,
  QueryProvider,
};
