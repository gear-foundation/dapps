import type { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { QueryProvider } from '@dapps-frontend/ui';

import { AppProvider, LessonsProvider, TmgProvider } from '../context';

import { AccountProvider, AlertProvider, ApiProvider } from './providers';

const providers = [
  BrowserRouter,
  AlertProvider,
  ApiProvider,
  AccountProvider,
  AppProvider,
  LessonsProvider,
  TmgProvider,
  QueryProvider,
];

export const withProviders = (Component: ComponentType) => () =>
  providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
