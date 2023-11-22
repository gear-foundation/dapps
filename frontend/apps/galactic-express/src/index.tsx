import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { initErrorTracking, logPublicEnvs, InitializeGoogleAnalytics } from '@dapps-frontend/error-tracking';
import { App } from './App';

InitializeGoogleAnalytics()
initErrorTracking();
logPublicEnvs();

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
