import { InjectedAccount, InjectedWindow } from '@polkadot/extension-inject/types';
import { SignerPayloadRaw } from '@polkadot/types/types';
import WebApp from '@twa-dev/sdk';

interface ConnectRequest {
  method: 'connect';
  callback: string;
}

interface SignTransactionRequest {
  method: 'signTransaction';
  tx: string;
  wallet: string;
  callback: string;
}

interface ConnectResult {
  success: boolean;
  wallets?: string[];
  error?: string;
}

interface SignTransactionResult {
  success: boolean;
  signedTx?: string;
  error?: string;
}

const VARAN_WALLET_CONNECTED_KEY = 'varanWalletsConnected';
const DEFAULT_CALLBACK = 'ZkPokerTestBot/zkPoker?startapp=';

const encodeRequest = (request: ConnectRequest | SignTransactionRequest): string => {
  const jsonString = JSON.stringify(request);
  const base64String = btoa(jsonString);
  return base64String.replace(/\//g, '_');
};

const decodeResult = (encodedResult: string): ConnectResult | SignTransactionResult => {
  try {
    const base64String = encodedResult.replace(/_/g, '/');
    const paddedBase64 = base64String + '='.repeat((4 - (base64String.length % 4)) % 4);
    const jsonString = atob(paddedBase64);
    return JSON.parse(jsonString) as ConnectResult | SignTransactionResult;
  } catch (error) {
    console.error('Error decoding result:', error);
    return { success: false, error: 'Failed to decode result' };
  }
};

const parseWalletsFromStartParam = () => {
  const startParam = WebApp.initDataUnsafe?.start_param;
  if (!startParam) return null;

  const result = decodeResult(startParam);
  console.log('parseWalletsFromStartParam', result);
  return 'wallets' in result ? result.wallets : null;
};

const parseSignedTxFromStartParam = () => {
  const startParam = WebApp.initDataUnsafe?.start_param;
  if (!startParam) return null;

  const result = decodeResult(startParam);
  console.log('parseSignedTxFromStartParam', result);
  return 'signedTx' in result ? result.signedTx : null;
};

const connect = () => {
  const request: ConnectRequest = {
    method: 'connect',
    callback: DEFAULT_CALLBACK,
  };

  const encodedRequest = encodeRequest(request);
  const botUrl = `https://t.me/devReptileBot?startapp=${encodedRequest}`;

  if (typeof window !== 'undefined') {
    window.open(botUrl, '_blank');
  }

  return Promise.resolve([]);
};

const signTransaction = (txHash: string, walletAddress: string, callbackUrl: string = DEFAULT_CALLBACK): void => {
  // Get Accural from pts 0x2858459a64019ccb05bad84eb3093dd1df12bf7b61cc97058aa30acaccd7e266
  // const txHash =
  //   '0x31010468032858459a64019ccb05bad84eb3093dd1df12bf7b61cc97058aa30acaccd7e2663c0c507473284765744163637572616c9253b798000000000000000000000000000000000000000001';
  // const walletAddress = 'kGk1iZDGu86wx5sE39bAvju1RWUoPwKgxx6i3u2oggEjCy4vF';
  const request: SignTransactionRequest = {
    method: 'signTransaction',
    tx: txHash,
    wallet: walletAddress,
    callback: callbackUrl,
  };

  const encodedRequest = encodeRequest(request);
  const botUrl = `https://t.me/devReptileBot?startapp=${encodedRequest}`;

  if (typeof window !== 'undefined') {
    // ! TODO: try WebApp.openLink
    window.open(botUrl, '_blank');
  }
};

const injectVaranWallet = () => {
  console.log('WebApp', WebApp);
  const isInTelegram = WebApp.platform !== 'unknown';

  if (!isInTelegram) return;

  const getInjectedAccount = (address: string): InjectedAccount => {
    return {
      address,
      name: address.slice(0, 6) + '...' + address.slice(-4),
    };
  };

  console.log('🚀 ~ injectVaranWallet ~ !!');
  (window as unknown as InjectedWindow).injectedWeb3 = {
    varan: {
      version: '1.0.0',
      connect: () =>
        Promise.resolve({
          name: 'varan',
          version: '1.0.0',
          accounts: {
            get: () =>
              new Promise((resolve, reject) => {
                const newWalletAddresses = parseWalletsFromStartParam();
                const connectedAccounts = JSON.parse(
                  localStorage.getItem(VARAN_WALLET_CONNECTED_KEY) || '[]',
                ) as string[];

                console.log('🚀 ~ injectVaranWal ~ newWalletAddresses:', newWalletAddresses);

                if (newWalletAddresses) {
                  const newConnectedAccounts = [...new Set([...connectedAccounts, ...newWalletAddresses])];
                  localStorage.setItem(VARAN_WALLET_CONNECTED_KEY, JSON.stringify(newConnectedAccounts));

                  return resolve(newConnectedAccounts.map(getInjectedAccount));
                }

                if (connectedAccounts.length > 0) {
                  return resolve(connectedAccounts.map(getInjectedAccount));
                }

                WebApp.showConfirm('Open Varan Wallet to connect account?', async (confirmed) => {
                  if (confirmed) {
                    const result = await connect();
                    resolve(result);
                  } else {
                    reject(new Error('User rejected'));
                  }
                });
              }),
            subscribe: () => () => {},
          },
          signer: {
            signRaw: ({ address, data }: SignerPayloadRaw) => {
              console.log('🚀 ~ signRaw ~ data:', data);
              return new Promise((resolve, reject) => {
                WebApp.showConfirm('Open Varan Wallet to sign transaction?', (confirmed) => {
                  if (confirmed) {
                    // !! TODO: get rawData from data
                    // In signRaw data has format:
                    // ('0x680364ea2fb67db50d1a420584f0f701cfdd427e6790757d8fde0676591035debcb63c0c507473284765744163637572616c9f06aa9a00000000000000000000000000000000000000000196000800001207000001000000525639f713f397dcf839bd022cd821f367ebcf179de7b9253531f8adbe5436d6df66964df7da51acebd267e83c59e95de22bdb32443f6d83139afcdaa90632b700');

                    // Wallet works with rawData format that get from transaction.extrinsic.toHex():
                    const rawData =
                      '0x310104680364ea2fb67db50d1a420584f0f701cfdd427e6790757d8fde0676591035debcb63c0c507473284765744163637572616cb311859a000000000000000000000000000000000000000001';

                    signTransaction(rawData, address);
                    resolve({
                      id: 0,
                      signature: '0x',
                    });
                  } else {
                    reject(new Error('User rejected'));
                  }
                });
              });
            },
          },
        }),
    },
  };
};

const varanWallet = {
  signTransaction,
  parseWalletsFromStartParam,
  parseSignedTxFromStartParam,
  injectVaranWallet,
};

export { varanWallet };
