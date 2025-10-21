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

import { DnsProvider as SharedDnsProvider, useDnsProgramIds } from '@dapps-frontend/hooks';
import { QueryProvider } from '@dapps-frontend/ui';

import { ENV, SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { usePokerProgram } from '@/app/utils';
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

const dnsNames = { pokerFactoryProgramId: ENV.DNS_NAME };

function DnsProvider({ children }: ProviderProps) {
  return (
    <SharedDnsProvider names={dnsNames} dnsApiUrl={ENV.DNS_API_URL}>
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
      allowedActions={SIGNLESS_ALLOWED_ACTIONS}
      voucherIssueAmount={ENV.SIGNLESS_VOUCHER_ISSUE_AMOUNT}
      voucherReissueThreshold={ENV.VOUCHER_LIMIT}
      allowIncreaseVoucherValue>
      {children}
    </SharedSignlessTransactionsProvider>
  );
}

const providers = [
  BrowserRouter,
  DnsProvider,
  ApiProvider,
  QueryProvider,
  AccountProvider,
  AlertProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
  EzTransactionsProvider,
];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
