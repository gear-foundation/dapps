import '@gear-js/vara-ui/dist/style-deprecated.css';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

import { App } from './App';

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
