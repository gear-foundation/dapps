import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './app';
import * as Sentry from '@sentry/react';
import { createRoutesFromChildren, matchRoutes, useLocation, useNavigationType } from 'react-router-dom';
import TagManager from 'react-gtm-module';
import { ADDRESS } from '@/app/consts';

if (import.meta.env.MODE === 'production' && import.meta.env.VITE_GTM_ID_TTT) {
  TagManager.initialize({
    gtmId: import.meta.env.VITE_GTM_ID_TTT,
  });
}

if (ADDRESS.SENTRY_DSN) {
  Sentry.init({
    dsn: ADDRESS.SENTRY_DSN,
    integrations: [
      new Sentry.BrowserTracing({
        routingInstrumentation: Sentry.reactRouterV6Instrumentation(
          React.useEffect,
          useLocation,
          useNavigationType,
          createRoutesFromChildren,
          matchRoutes,
        ),
      }),
      new Sentry.Replay({
        maskAllText: false,
      }),
    ],
    // Set 'tracePropagationTargets' to control for which URLs distributed tracing should be enabled
    tracePropagationTargets: [
      // 'localhost',
      /^https:\/\/cb-tic-tac-toe\.vara-network\.io/,
    ],
    // Performance Monitoring
    tracesSampleRate: 0.1, // Capture 100% of the transactions, reduce in production!
    // Session Replay
    replaysSessionSampleRate: 0.1, // This sets the sample rate at 10%. You may want to change it to 100% while in development and then sample at a lower rate in production.
    replaysOnErrorSampleRate: 1.0, // If you're not already sampling the entire session, change the sample rate to 100% when sampling sessions where errors occur.
  });
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
