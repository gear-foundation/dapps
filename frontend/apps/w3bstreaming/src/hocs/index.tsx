import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { QueryProvider } from '@dapps-frontend/ui';

import { AccountProvider, AlertProvider, ApiProvider, DnsProvider } from './providers';

const providers = [BrowserRouter, AlertProvider, ApiProvider, DnsProvider, AccountProvider, QueryProvider];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
