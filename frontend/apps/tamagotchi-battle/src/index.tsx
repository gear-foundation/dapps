import React from 'react';
import ReactDOM from 'react-dom/client';
import TagManager from 'react-gtm-module';
import { initErrorTracking, logPublicEnvs } from 'error-tracking';
import { App } from './app';

if (process.env.MODE === 'production' && process.env.VITE_GTM_ID_TTT) {
  TagManager.initialize({
    gtmId: process.env.VITE_GTM_ID_TTT,
  });
}

initErrorTracking();
logPublicEnvs();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
