# @dapps-frontend/error-tracking

Error tracking and monitoring utils.

## Install:

```sh
yarn add @dapps-frontend/error-tracking
```

## Error tracking

### Configure:

Specify Sentry environment variables:

```sh
REACT_APP_SENTRY_DSN=
REACT_APP_SENTRY_TARGET=
```

or

```sh
VITE_SENTRY_DSN=
VITE_SENTRY_TARGET=
```

Where `DSN` is [Data Source Name](https://docs.sentry.io/product/sentry-basics/concepts/dsn-explainer/) and `TARGET` is [tracePropagationTargets](https://docs.sentry.io/platforms/javascript/performance/instrumentation/automatic-instrumentation/#tracepropagationtargets).

If `TARGET` is not provided, `localhost` will be used by default.

### Use:

In the root of your application, call `initErrorTracking` function:

```jsx
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { initErrorTracking } from '@dapps-frontend/error-tracking';
import { App } from './App';

initErrorTracking();

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
```

Wrap your routes with `ErrorTrackingRoutes`:

```jsx
import { Route } from 'react-router-dom';
import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';
import { Home } from './home';

const routes = [{ path: '/', Page: Home }];

function Routing() {
  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  return <ErrorTrackingRoutes>{getRoutes()}</ErrorTrackingRoutes>;
}

export { Routing };
```

Additionally, you can use `ErrorBoundary` component or `withErrorBoundary` HOC.

## Public envs logging

For dev and debug purposes. Be careful to only share public information, such as chain/contract addresses.

### Configure

It's recommended to stick to below varaible names for chain values, since they'll be logged by default:

```sh
REACT_APP_NODE_ADDRESS=
REACT_APP_CONTRACT_ADDRESS=
REACT_APP_IPFS_ADDRESS=
REACT_APP_IPFS_GATEWAY_ADDRESS=
```

or

```sh
VITE_NODE_ADDRESS=
VITE_CONTRACT_ADDRESS=
VITE_IPFS_ADDRESS=
VITE_IPFS_GATEWAY_ADDRESS=
```

### Use

To log public environment variables to console, call `logPublicEnvs` function in the root of your application:

```jsx
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { logPublicEnvs } from '@dapps-frontend/error-tracking';
import { App } from './App';

logPublicEnvs();

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
```

In case of custom variable names, function accepts object:

```ts
logPublicEnvs({ marketplaceContractAddress });
```
