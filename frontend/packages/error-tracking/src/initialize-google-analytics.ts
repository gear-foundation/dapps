import TagManager from 'react-gtm-module';
import { getCRAEnv, getViteEnv } from './utils';

function initAnalytics() {
  const gtm = getCRAEnv('GTM_ID') || getViteEnv('GTM_ID');

  if (gtm) {
    TagManager.initialize({
      gtmId: gtm,
    });
  }
}

export { initAnalytics };
