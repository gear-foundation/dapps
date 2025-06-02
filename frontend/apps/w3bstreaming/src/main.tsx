import { createRoot } from 'react-dom/client';

import { initErrorTracking, logPublicEnvs, initAnalytics } from '@dapps-frontend/error-tracking';

import { App } from '@/App';
import './styles/global.scss';

initAnalytics();
initErrorTracking();
logPublicEnvs();

const container = document.getElementById('root');
const root = createRoot(container as Element);

root.render(<App />);
