import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

import { initErrorTracking, logPublicEnvs, initAnalytics } from '@dapps-frontend/error-tracking';

import { ENV } from '@/consts';

import { App } from './App';

initAnalytics();
initErrorTracking();
logPublicEnvs({ marketplaceContract: ENV.MARKETPLACE_CONTRACT, nftContract: ENV.NFT_CONTRACT });

const container = document.getElementById('root');
const root = createRoot(container as HTMLElement);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
);
