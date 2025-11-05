import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { QueryProvider } from '@dapps-frontend/ui';

import { IPFSProvider } from '@/context';

import { AccountProvider, AlertProvider, ApiProvider } from './providers';

const providers = [BrowserRouter, AlertProvider, IPFSProvider, ApiProvider, AccountProvider, QueryProvider];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
