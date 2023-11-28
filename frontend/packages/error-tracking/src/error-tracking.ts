import {
  BrowserTracing,
  ErrorBoundary,
  Replay,
  init,
  reactRouterV6Instrumentation,
  withErrorBoundary,
  withSentryReactRouterV6Routing,
} from '@sentry/react';
import { useEffect } from 'react';
import { Routes, createRoutesFromChildren, matchRoutes, useLocation, useNavigationType } from 'react-router-dom';
import { getCRAEnv, getViteEnv } from './utils';

function initErrorTracking() {
  const dsn = getCRAEnv('SENTRY_DSN') || getViteEnv('SENTRY_DSN');
  const target = getCRAEnv('SENTRY_TARGET') || getViteEnv('SENTRY_TARGET') || 'localhost';

  // See docs for support of different versions of variation of react router
  // https://docs.sentry.io/platforms/javascript/guides/react/configuration/integrations/react-router/
  const routingInstrumentation = reactRouterV6Instrumentation(
    useEffect,
    useLocation,
    useNavigationType,
    createRoutesFromChildren,
    matchRoutes,
  );

  const integrations = [new BrowserTracing({ routingInstrumentation }), new Replay()];

  // Set `tracePropagationTargets` to control for which URLs distributed tracing should be enabled
  const tracePropagationTargets = [target];

  // Set tracesSampleRate to 1.0 to capture 100%
  // of transactions for performance monitoring.
  const tracesSampleRate = 0.1;

  // Capture Replay for 10% of all sessions,
  // plus for 100% of sessions with an error
  const replaysSessionSampleRate = 0.1;
  const replaysOnErrorSampleRate = 1.0;

  init({
    dsn,
    integrations,
    tracePropagationTargets,
    tracesSampleRate,
    replaysSessionSampleRate,
    replaysOnErrorSampleRate,
  });
}

const ErrorTrackingRoutes = withSentryReactRouterV6Routing(Routes);

export { initErrorTracking, ErrorTrackingRoutes, ErrorBoundary, withErrorBoundary };
