import { EzTransactionsProvider } from 'gear-ez-transactions';
import type { ComponentType } from 'react';

import { QueryProvider } from '@dapps-frontend/ui';

import { AppProvider } from '@/app/context/ctx-app';
import { GameProvider } from '@/app/context/ctx-game';

import {
  BrowserRouterProvider,
  AlertProvider,
  ApiProvider,
  DnsProvider,
  AccountProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
} from './providers';

const providers = [
  BrowserRouterProvider,
  AlertProvider,
  ApiProvider,
  DnsProvider,
  AccountProvider,
  QueryProvider,
  AppProvider,
  GameProvider,
  GaslessTransactionsProvider,
  SignlessTransactionsProvider,
  EzTransactionsProvider,
];

export const withProviders = (Component: ComponentType) => () =>
  providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
