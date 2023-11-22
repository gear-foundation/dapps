import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import TagManager from 'react-gtm-module';
import { initErrorTracking, logPublicEnvs, InitializeGoogleAnalytics } from '@dapps-frontend/error-tracking';
import 'keen-slider/keen-slider.min.css';
import { App } from './app';


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
