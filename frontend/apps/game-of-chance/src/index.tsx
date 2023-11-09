import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { initErrorTracking } from 'error-tracking';
import { App } from './App';

initErrorTracking();

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
