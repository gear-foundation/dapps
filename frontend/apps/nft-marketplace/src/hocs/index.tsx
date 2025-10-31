import { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { QueryProvider } from '@dapps-frontend/ui';

import { IPFSProvider } from '@/context';

import { ApiProvider, AccountProvider, AlertProvider } from './providers';

const providers = [BrowserRouter, IPFSProvider, AlertProvider, ApiProvider, AccountProvider, QueryProvider];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
