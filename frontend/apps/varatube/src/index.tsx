import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { initErrorTracking, logPublicEnvs, InitializeGoogleAnalytics } from '@dapps-frontend/error-tracking';
import { ADDRESS } from 'consts';
import { App } from './App';

InitializeGoogleAnalytics();
initErrorTracking();
logPublicEnvs({ ftContract: ADDRESS.FT_CONTRACT });

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
