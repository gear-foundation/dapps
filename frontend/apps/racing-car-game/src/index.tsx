import { createRoot } from 'react-dom/client';
import TagManager from 'react-gtm-module';
import { initErrorTracking, logPublicEnvs } from 'error-tracking';
import { App } from '@/App';
import './styles/global.scss';

if (process.env.NODE_ENV === 'production' && process.env.REACT_APP_GTM_ID_CARS) {
  TagManager.initialize({
    gtmId: process.env.REACT_APP_GTM_ID_CARS,
  });
}

initErrorTracking();
logPublicEnvs();

const container = document.getElementById('root');
const root = createRoot(container as Element);

root.render(<App />);
