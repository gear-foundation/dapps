import { ComponentType } from 'react';

import {
  BrowserRouter,
  ApiProvider,
  AccountProvider,
  AlertProvider,
  DnsProvider,
  QueryProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
  EzTransactionsProvider,
} from './providers';

export const providers = [
  BrowserRouter,
  ApiProvider,
  AccountProvider,
  AlertProvider,
  DnsProvider,
  QueryProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
  EzTransactionsProvider,
];

function withProviders(Component: ComponentType) {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
}

export { withProviders };
