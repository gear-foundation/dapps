import TagManager from 'react-gtm-module';

import { getViteEnv } from './utils';

function initAnalytics() {
  const gtmId = getViteEnv('GTM_ID');
  if (!gtmId) return;

  TagManager.initialize({ gtmId });
}

export { initAnalytics };
