import TagManager from 'react-gtm-module';
import { getCRAEnv, getViteEnv } from './utils';

function InitializeGoogleAnalytics() {
  const gtm = getCRAEnv('GTM_ID') || getViteEnv('GTM_ID');

  if (process.env.NODE_ENV === 'production' && gtm) {
    TagManager.initialize({
      gtmId: gtm,
    });
  }
}

export { InitializeGoogleAnalytics };
