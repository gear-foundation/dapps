import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { AvailableBalanceProvider } from '@dapps-frontend/hooks';
import { QueryProvider } from '@dapps-frontend/ui';

import { ApiProvider, AccountProvider, AlertProvider } from './providers';

const providers = [BrowserRouter, AlertProvider, ApiProvider, AccountProvider, AvailableBalanceProvider, QueryProvider];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
