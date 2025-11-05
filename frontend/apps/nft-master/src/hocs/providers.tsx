import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider as GearAccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { Provider as UrqlClientProvider } from 'urql';

import { DnsProvider as SharedDnsProvider } from '@dapps-frontend/hooks';

import { Alert, alertStyles } from '@/components/ui/alert';
import { ENV } from '@/consts';
import { urqlClient } from '@/utils';

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider initialArgs={{ endpoint: ENV.DEFAULT_NODE }}>{children}</GearApiProvider>
);

const AccountProvider = ({ children }: ProviderProps) => (
  <GearAccountProvider appName="Vara NFT Portal">{children}</GearAccountProvider>
);

const DnsProvider = ({ children }: ProviderProps) => (
  <SharedDnsProvider names={{ programId: ENV.DNS_NAME }} dnsApiUrl={ENV.DNS_API_URL}>
    {children}
  </SharedDnsProvider>
);

const AlertProvider = ({ children }: ProviderProps) => (
  <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
    {children}
  </GearAlertProvider>
);

const UrqlProvider = ({ children }: ProviderProps) => (
  <UrqlClientProvider value={urqlClient}>{children}</UrqlClientProvider>
);

export { ApiProvider, AccountProvider, AlertProvider, DnsProvider, UrqlProvider };
