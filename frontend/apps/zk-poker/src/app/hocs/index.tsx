import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider as GearAccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import {
  // SignlessTransactionsProvider as SharedSignlessTransactionsProvider,
  GaslessTransactionsProvider as SharedGaslessTransactionsProvider,
  EzTransactionsProvider,
} from 'gear-ez-transactions';
import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';
import { Provider as UrqlClientProvider } from 'urql';

import { DnsProvider as SharedDnsProvider, useDnsProgramIds } from '@dapps-frontend/hooks';
import { QueryProvider } from '@dapps-frontend/ui';

import { ADDRESS } from '@/app/consts';
import { urqlClient } from '@/app/utils';
import { Alert, alertStyles } from '@/components/ui/alert';

// ! TODO: add program
// import { useProgram } from '../utils/sails';

function ApiProvider({ children }: ProviderProps) {
  return <GearApiProvider initialArgs={{ endpoint: ADDRESS.NODE }}>{children}</GearApiProvider>;
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
    <SharedDnsProvider names={{ pokerFactoryProgramId: ADDRESS.DNS_NAME }} dnsApiUrl={ADDRESS.DNS_API_URL}>
      {children}
    </SharedDnsProvider>
  );
}

function GaslessTransactionsProvider({ children }: ProviderProps) {
  const { pokerFactoryProgramId } = useDnsProgramIds<'pokerFactoryProgramId'>();

  return (
    <SharedGaslessTransactionsProvider
      programId={pokerFactoryProgramId}
      backendAddress={ADDRESS.GASLESS_BACKEND}
      voucherLimit={18}>
      {children}
    </SharedGaslessTransactionsProvider>
  );
}

// function SignlessTransactionsProvider({ children }: ProviderProps) {
//   // ! TODO: add program
//   const { programId } = useDnsProgramIds();
//   const program = useProgram();

//   return (
//     <SharedSignlessTransactionsProvider programId={programId} program={program}>
//       {children}
//     </SharedSignlessTransactionsProvider>
//   );
// }

function UrqlProvider({ children }: ProviderProps) {
  return <UrqlClientProvider value={urqlClient}>{children}</UrqlClientProvider>;
}

const providers = [
  BrowserRouter,
  UrqlProvider,
  ApiProvider,
  AccountProvider,
  AlertProvider,
  QueryProvider,
  DnsProvider,
  GaslessTransactionsProvider,
  // SignlessTransactionsProvider,
  EzTransactionsProvider,
];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
