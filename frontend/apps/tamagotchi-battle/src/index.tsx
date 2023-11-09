import React from 'react';
import ReactDOM from 'react-dom/client';
import { InitErrorTracking } from 'error-tracking';
import { App } from './app';

InitErrorTracking();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
