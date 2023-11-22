import { createRoot } from 'react-dom/client';
import TagManager from 'react-gtm-module';
import { initErrorTracking, logPublicEnvs, InitializeGoogleAnalytics } from '@dapps-frontend/error-tracking';
import { App } from '@/App';
import './styles/global.scss';

InitializeGoogleAnalytics()
initErrorTracking();
logPublicEnvs();

const container = document.getElementById('root');
const root = createRoot(container as Element);

root.render(<App />);
