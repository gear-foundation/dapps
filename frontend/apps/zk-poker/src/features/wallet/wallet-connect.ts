import WebApp from '@twa-dev/sdk';
import { WalletConnectModal } from '@walletconnect/modal';
import { UniversalProvider } from '@walletconnect/universal-provider';

async function connectWallet() {
  try {
    const provider = await UniversalProvider.init({
      projectId: '7d8f025850f5f8c5cd9266b496ada948',
      relayUrl: 'wss://relay.walletconnect.com',
    });

    const params = {
      requiredNamespaces: {
        polkadot: {
          methods: ['polkadot_signTransaction', 'polkadot_signMessage'],
          chains: [
            // 'polkadot:91b171bb158e2d3848fa23a9f1c25182', // polkadot
            // 'polkadot:525639f713f397dcf839bd022cd821f3', // vara testnet
            'polkadot:fe1b4c55fd4d668101126434206571a7', // vara mainnet
          ],
          events: ['chainChanged', 'accountsChanged'],
        },
      },
    };

    const { uri, approval } = await provider.client.connect(params);

    const walletConnectModal = new WalletConnectModal({
      projectId: '7d8f025850f5f8c5cd9266b496ada948',
      explorerRecommendedWalletIds: [
        // https://walletguide.walletconnect.network/?chains=polkadot%3A91b171bb158e2d3848fa23a9f1c25182
        '9ce87712b99b3eb57396cc8621db8900ac983c712236f48fb70ad28760be3f6a', // subwallet
        '43fd1a0aeb90df53ade012cca36692a46d265f0b99b7561e645af42d752edb92', // nova
        'e0c2e199712878ed272e2c170b585baa0ff0eb50b07521ca586ebf7aeeffc598', // talisman
      ],
    });

    if (uri) {
      void walletConnectModal.openModal({ uri });

      const walletConnectSession = await approval();
      console.log('🚀 ~ connectWallet ~ walletConnectSession :', walletConnectSession);

      const walletConnectAccount = Object.values(walletConnectSession.namespaces)
        .map((namespace) => namespace.accounts)
        .flat();
      console.log('🚀 ~ connectWallet ~ walletConnectAccount:', walletConnectAccount);

      // grab account addresses from CAIP account formatted accounts
      const accounts = walletConnectAccount.map((wcAccount) => {
        const address = wcAccount.split(':')[2];
        return address;
      });
      console.log('🚀 ~ accounts ~ accounts:', accounts);
      WebApp.showAlert(walletConnectAccount.toString());

      walletConnectModal.closeModal();
      return walletConnectSession;
    }
  } catch (error) {
    console.error(error);
  }
}

export default connectWallet;
