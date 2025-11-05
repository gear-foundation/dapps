import type { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import {
  AccountProvider,
  AlertProvider,
  ApiProvider,
  BattleProvider,
  DnsProvider,
  GaslessTransactionsProvider,
  QueryProvider,
} from './providers';

const providers = [
  BrowserRouter,
  AlertProvider,
  ApiProvider,
  QueryProvider,
  DnsProvider,
  AccountProvider,
  BattleProvider,
  GaslessTransactionsProvider,
];

const withProviders = (Component: ComponentType) => {
  return () => providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
};

export { withProviders };
