import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider as GearAccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { DnsProvider as SharedDnsProvider } from '@dapps-frontend/hooks';
import { QueryProvider } from '@dapps-frontend/ui';
import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';
import { Provider as UrqlClientProvider } from 'urql';
import { Alert, alertStyles } from 'components/ui/alert';
import { urqlClient } from 'utils';
import { ADDRESS } from '../consts';

function ApiProvider({ children }: ProviderProps) {
  return <GearApiProvider initialArgs={{ endpoint: ADDRESS.DEFAULT_NODE }}>{children}</GearApiProvider>;
}

function AccountProvider({ children }: ProviderProps) {
  return <GearAccountProvider appName="Vara NFT Portal">{children}</GearAccountProvider>;
}

function DnsProvider({ children }: ProviderProps) {
  return (
    <SharedDnsProvider names={{ programId: ADDRESS.DNS_NAME }} dnsApiUrl={ADDRESS.DNS_API_URL}>
      {children}
    </SharedDnsProvider>
  );
}

function AlertProvider({ children }: ProviderProps) {
  return (
    <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
      {children}
    </GearAlertProvider>
  );
}

function UrqlProvider({ children }: ProviderProps) {
  return <UrqlClientProvider value={urqlClient}>{children}</UrqlClientProvider>;
}

const providers = [
  BrowserRouter,
  UrqlProvider,
  AlertProvider,
  ApiProvider,
  DnsProvider,
  AccountProvider,
  QueryProvider,
];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
