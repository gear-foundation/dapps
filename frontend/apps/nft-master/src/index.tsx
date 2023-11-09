import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import TagManager from 'react-gtm-module';
import { initErrorTracking } from 'error-tracking';
import 'keen-slider/keen-slider.min.css';
import { App } from './app';

if (process.env.NODE_ENV === 'production' && process.env.REACT_APP_GTM_ID) {
  TagManager.initialize({
    gtmId: process.env.REACT_APP_GTM_ID,
  });
}

initErrorTracking();

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
