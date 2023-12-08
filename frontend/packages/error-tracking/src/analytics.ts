import TagManager from 'react-gtm-module';
import { getCRAEnv, getViteEnv } from './utils';

function initAnalytics() {
  const gtmId = getCRAEnv('GTM_ID') || getViteEnv('GTM_ID');
  if (!gtmId) return;

  TagManager.initialize({ gtmId });
}

export { initAnalytics };
