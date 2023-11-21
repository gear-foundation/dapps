import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';
import { Provider as UrqlClientProvider } from 'urql';
import { Alert, alertStyles } from 'components/ui/alert';
import { urqlClient } from 'utils';
import { ADDRESS } from '../consts';

function ApiProvider({ children }: ProviderProps) {
  return <GearApiProvider initialArgs={{ endpoint: ADDRESS.DEFAULT_NODE }}>{children}</GearApiProvider>;
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

const providers = [BrowserRouter, UrqlProvider, AlertProvider, ApiProvider, AccountProvider];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
