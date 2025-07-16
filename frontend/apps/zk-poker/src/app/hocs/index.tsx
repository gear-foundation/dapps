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
import { Provider as UrqlClientProvider } from 'urql';

import { DnsProvider as SharedDnsProvider, useDnsProgramIds } from '@dapps-frontend/hooks';
import { QueryProvider } from '@dapps-frontend/ui';

import { ENV } from '@/app/consts';
import { urqlClient, usePokerProgram } from '@/app/utils';
import { Alert, alertStyles } from '@/components/ui/alert';

function ApiProvider({ children }: ProviderProps) {
  return <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>;
}

function AccountProvider({ children }: ProviderProps) {
  return <GearAccountProvider appName="Vara ZK Poker">{children}</GearAccountProvider>;
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
    <SharedDnsProvider
      names={{ pokerFactoryProgramId: ENV.DNS_NAME }}
      dnsApiUrl={ENV.DNS_API_URL}
      forcedValue={{ pokerFactoryProgramId: ENV.FORCED_POKER_FACTORY_PROGRAM_ID }}>
      {children}
    </SharedDnsProvider>
  );
}

function GaslessTransactionsProvider({ children }: ProviderProps) {
  const { pokerFactoryProgramId } = useDnsProgramIds<'pokerFactoryProgramId'>();

  return (
    <SharedGaslessTransactionsProvider
      programId={pokerFactoryProgramId}
      backendAddress={ENV.GASLESS_BACKEND}
      voucherLimit={Number(ENV.VOUCHER_LIMIT)}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

function SignlessTransactionsPokerProvider({ children }: ProviderProps) {
  const program = usePokerProgram();

  if (!program?.programId) return children;

  return (
    <SharedSignlessTransactionsProvider
      programId={program?.programId}
      program={program}
      voucherIssueAmount={Number(ENV.SIGNLESS_VOUCHER_ISSUE_AMOUNT)}
      voucherReissueThreshold={Number(ENV.VOUCHER_LIMIT)}>
      {children}
    </SharedSignlessTransactionsProvider>
  );
}

function UrqlProvider({ children }: ProviderProps) {
  return <UrqlClientProvider value={urqlClient}>{children}</UrqlClientProvider>;
}

const providers = [
  BrowserRouter,
  DnsProvider,
  ApiProvider,
  QueryProvider,
  UrqlProvider,
  AccountProvider,
  AlertProvider,
  GaslessTransactionsProvider,
];

// programId only available on game route
const gameRouteProviders = [SignlessTransactionsPokerProvider, EzTransactionsProvider];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

function withGameRouteProviders(Component: ComponentType) {
  return () => gameRouteProviders.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders, withGameRouteProviders };
