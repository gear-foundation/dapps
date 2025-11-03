import type { ComponentType } from 'react';

import { providers } from './providers';

export const withProviders = (Component: ComponentType) => () =>
  providers.reduceRight((children, Provider) => <Provider>{children}</Provider>, <Component />);
