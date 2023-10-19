import { StrictMode, useEffect } from 'react'
import { createRoot } from 'react-dom/client'
import 'keen-slider/keen-slider.min.css'
import * as Sentry from '@sentry/react'
import {
  createRoutesFromChildren,
  matchRoutes,
  useLocation,
  useNavigationType,
} from 'react-router-dom'
import TagManager from 'react-gtm-module'
import { App } from './app'
import { ADDRESS } from './consts'

if (process.env.NODE_ENV === 'production' && process.env.REACT_APP_GTM_ID) {
  TagManager.initialize({
    gtmId: process.env.REACT_APP_GTM_ID,
  })
}
if (ADDRESS.SENTRY_DSN) {
  Sentry.init({
    dsn: ADDRESS.SENTRY_DSN,
    integrations: [
      new Sentry.BrowserTracing({
        routingInstrumentation: Sentry.reactRouterV6Instrumentation(
          useEffect,
          useLocation,
          useNavigationType,
          createRoutesFromChildren,
          matchRoutes
        ),
      }),
      new Sentry.Replay({
        maskAllText: false
      }),
    ],
    // Set 'tracePropagationTargets' to control for which URLs distributed tracing should be enabled
    tracePropagationTargets: [
      // 'localhost',
      /^https:\/\/cb-nft\.vara-network\.io/,
    ],
    // Performance Monitoring
    tracesSampleRate: 0.1, // Capture 100% of the transactions, reduce in production!
    // Session Replay
    replaysSessionSampleRate: 0.1, // This sets the sample rate at 10%. You may want to change it to 100% while in development and then sample at a lower rate in production.
    replaysOnErrorSampleRate: 1.0, // If you're not already sampling the entire session, change the sample rate to 100% when sampling sessions where errors occur.
  })
}

const container = document.getElementById('root')
const root = createRoot(container as HTMLElement)

root.render(
  <StrictMode>
    <App />
  </StrictMode>
)
