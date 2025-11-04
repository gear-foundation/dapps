import {
  AccountProvider as GearAccountProvider,
  AlertProvider as GearAlertProvider,
  ApiProvider as GearApiProvider,
  ProviderProps,
} from '@gear-js/react-hooks';
import { Alert, alertStyles } from '@gear-js/ui';
import { JSX } from 'react';

import { ENV } from '../consts';

export function ApiProvider({ children }: ProviderProps): JSX.Element {
  return <GearApiProvider initialArgs={{ endpoint: ENV.NODE }}>{children}</GearApiProvider>;
}

export function AccountProvider({ children }: ProviderProps): JSX.Element {
  return <GearAccountProvider appName="Vara Tamagotchi">{children}</GearAccountProvider>;
}

export function AlertProvider({ children }: ProviderProps): JSX.Element {
  return (
    <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
      {children}
    </GearAlertProvider>
  );
}
