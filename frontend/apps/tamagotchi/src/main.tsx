import React from 'react';
import ReactDOM from 'react-dom/client';
import { initErrorTracking, logPublicEnvs, InitializeGoogleAnalytics } from '@dapps-frontend/error-tracking';
import { App } from './app';

InitializeGoogleAnalytics();
initErrorTracking();
logPublicEnvs();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
