import { useApi } from '@gear-js/react-hooks';
import WebApp from '@twa-dev/sdk';
import { useEffect } from 'react';

import { varanWallet } from './varan-wallet';

const useVaranWallet = () => {
  const { api } = useApi();

  useEffect(() => {
    const signedTx = varanWallet.parseSignedTxFromStartParam();

    if (signedTx && api) {
      WebApp.showConfirm('Send signed by Varan Wallet transaction?', (confirmed) => {
        if (confirmed) {
          void api.rpc.author
            .submitExtrinsic(signedTx)
            .then((result) => {
              console.log('🚀 ~ submitExtrinsic ~ result:', result);
              WebApp.showAlert('Transaction sent successfully');
            })
            .catch((error) => {
              console.log('🚀 ~ submitExtrinsic ~ error:', error);
              WebApp.showAlert('Transaction failed');
            });
        } else {
          console.log('User rejected');
        }
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const signedTx = varanWallet.parseSignedTxFromStartParam();

  return { signedTx };
};

export { useVaranWallet };
