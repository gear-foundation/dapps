import React from 'react';
import ReactDOM from 'react-dom/client';
import { initErrorTracking } from 'error-tracking';
import { App } from './app';

initErrorTracking();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
