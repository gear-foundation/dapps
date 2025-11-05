import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { QueryProvider } from '@dapps-frontend/ui';

import { AccountProvider, AlertProvider, ApiProvider, DnsProvider, UrqlProvider } from './providers';

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
