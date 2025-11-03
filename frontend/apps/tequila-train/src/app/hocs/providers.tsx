import type { ComponentType, PropsWithChildren } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { QueryProvider } from '@dapps-frontend/ui';

import { AppProvider, GameProvider } from '@/app/context';

import { AccountProvider, AlertProvider, ApiProvider, DnsProvider } from './provider-components';

const providers: ComponentType<PropsWithChildren>[] = [
  BrowserRouter,
  AlertProvider,
  ApiProvider,
  DnsProvider,
  AccountProvider,
  AppProvider,
  GameProvider,
  QueryProvider,
];

export { providers };
