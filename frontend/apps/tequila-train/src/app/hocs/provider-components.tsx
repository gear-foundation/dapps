import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider as GearAccountProvider,
} from '@gear-js/react-hooks';
import type { ProviderProps } from '@gear-js/react-hooks';

import { DnsProvider as SharedDnsProvider } from '@dapps-frontend/hooks';

import { ENV } from '@/app/consts';
import { Alert, alertStyles } from '@/components/ui/alert';

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>
);

const AccountProvider = ({ children }: ProviderProps) => (
  <GearAccountProvider appName="Vara Tequila Train">{children}</GearAccountProvider>
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

export { AccountProvider, AlertProvider, ApiProvider, DnsProvider };
