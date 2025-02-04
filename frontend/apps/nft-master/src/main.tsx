import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { initErrorTracking, logPublicEnvs, initAnalytics } from '@dapps-frontend/error-tracking';
import 'keen-slider/keen-slider.min.css';
import '@gear-js/vara-ui/dist/style.css';
import { App } from './app';

initAnalytics();
initErrorTracking();
logPublicEnvs();

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
