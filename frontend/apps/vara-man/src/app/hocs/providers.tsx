import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider as GearAccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import {
  SignlessTransactionsProvider as SharedSignlessTransactionsProvider,
  GaslessTransactionsProvider as SharedGaslessTransactionsProvider,
} from 'gear-ez-transactions';
import { BrowserRouter } from 'react-router-dom';

import { DnsProvider as SharedDnsProvider, useDnsProgramIds } from '@dapps-frontend/hooks';

import { ENV } from '@/app/consts';
import { Alert, alertStyles } from '@/components/ui/alert';

import { useProgram } from '../utils';

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>
);

function AccountProvider({ children }: ProviderProps) {
  return <GearAccountProvider appName="Vara-Man">{children}</GearAccountProvider>;
}

function DnsProvider({ children }: ProviderProps) {
  return (
    <SharedDnsProvider names={{ programId: ENV.DNS_NAME }} dnsApiUrl={ENV.DNS_API_URL}>
      {children}
    </SharedDnsProvider>
  );
}

const AlertProvider = ({ children }: ProviderProps) => (
  <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
    {children}
  </GearAlertProvider>
);

const BrowserRouterProvider = ({ children }: ProviderProps) => <BrowserRouter>{children}</BrowserRouter>;

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

export {
  ApiProvider,
  AccountProvider,
  DnsProvider,
  AlertProvider,
  BrowserRouterProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
};
