/* eslint-disable react-refresh/only-export-components */

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
import { AutoSignlessProvider } from '@/features/signless';

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
    <SharedDnsProvider names={{ pokerFactoryProgramId: ENV.DNS_NAME }} dnsApiUrl={ENV.DNS_API_URL}>
      {children}
    </SharedDnsProvider>
  );
}

function GaslessTransactionsProvider({ children }: ProviderProps) {
  const { pokerFactoryProgramId } = useDnsProgramIds<'pokerFactoryProgramId'>();
  const program = usePokerProgram();

  return (
    <SharedGaslessTransactionsProvider
      programId={program?.programId || pokerFactoryProgramId}
      backendAddress={ENV.GASLESS_BACKEND}
      voucherLimit={Number(ENV.VOUCHER_LIMIT)}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

function SignlessTransactionsProvider({ children }: ProviderProps) {
  const program = usePokerProgram();

  return (
    <SharedSignlessTransactionsProvider
      programId={program?.programId || '0x'}
      program={program}
      voucherIssueAmount={ENV.SIGNLESS_VOUCHER_ISSUE_AMOUNT}
      voucherReissueThreshold={ENV.VOUCHER_LIMIT}>
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
  SignlessTransactionsProvider,
  EzTransactionsProvider,
  AutoSignlessProvider,
];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
