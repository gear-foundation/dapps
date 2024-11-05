import type { ComponentType } from 'react';
import { BrowserRouter } from 'react-router-dom';
import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider as GearAccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { Alert, alertStyles } from '@gear-js/ui';
import { ENV } from '../consts';
import { AppProvider, LessonsProvider, TmgProvider } from '../context';
import { QueryProvider } from '@dapps-frontend/ui';

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>
);

function AccountProvider({ children }: ProviderProps) {
  return <GearAccountProvider appName="Vara Tamagotchi">{children}</GearAccountProvider>;
}

const AlertProvider = ({ children }: ProviderProps) => (
  <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
    {children}
  </GearAlertProvider>
);

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
