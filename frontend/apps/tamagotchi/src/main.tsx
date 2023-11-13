import React from 'react';
import ReactDOM from 'react-dom/client';
import { initErrorTracking, logPublicEnvs } from 'error-tracking';
import { App } from './app';

initErrorTracking();
logPublicEnvs();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
