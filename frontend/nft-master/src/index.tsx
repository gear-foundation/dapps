import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import 'keen-slider/keen-slider.min.css';
import TagManager from 'react-gtm-module';
import { App } from './App';

if (process.env.NODE_ENV === 'production') {
  TagManager.initialize({
    gtmId: 'GTM-PHHRZ89C',
  });
}

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
