import '@gear-js/vara-ui/dist/style.css';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { initErrorTracking, logPublicEnvs, initAnalytics } from '@dapps-frontend/error-tracking';
import { App } from './app';

initAnalytics();
initErrorTracking();
logPublicEnvs();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
