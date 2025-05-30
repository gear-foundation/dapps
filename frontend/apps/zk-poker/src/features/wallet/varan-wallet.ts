import { useAccount } from '@gear-js/react-hooks';
import { useCallback } from 'react';

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
  wallet?: string;
  error?: string;
}

interface SignTransactionResult {
  success: boolean;
  signature?: string;
  error?: string;
}

const useVaranWallet = () => {
  const { account } = useAccount();

  const encodeRequest = useCallback((request: ConnectRequest | SignTransactionRequest): string => {
    const jsonString = JSON.stringify(request);
    const base64String = btoa(jsonString);
    return base64String.replace(/\//g, '_');
  }, []);

  const decodeResult = useCallback((encodedResult: string): ConnectResult | SignTransactionResult => {
    try {
      const base64String = encodedResult.replace(/_/g, '/');
      const paddedBase64 = base64String + '='.repeat((4 - (base64String.length % 4)) % 4);
      const jsonString = atob(paddedBase64);
      return JSON.parse(jsonString) as ConnectResult | SignTransactionResult;
    } catch (error) {
      console.error('Error decoding result:', error);
      return { success: false, error: 'Failed to decode result' };
    }
  }, []);

  const connect = useCallback(
    (callbackUrl: string = 'http://localhost:3000/'): void => {
      const request: ConnectRequest = {
        method: 'connect',
        callback: callbackUrl,
      };

      const encodedRequest = encodeRequest(request);
      console.log('🚀 ~ useVaranWal ~ encodedRequest:', encodedRequest);
      const botUrl = `https://t.me/devReptileBot?startapp=${encodedRequest}`;

      if (typeof window !== 'undefined') {
        window.open(botUrl, '_blank');
      }
    },
    [encodeRequest],
  );

  const signTransaction = useCallback(
    (txHash: string, walletAddress: string, callbackUrl: string = 'wallet?startapp='): void => {
      const request: SignTransactionRequest = {
        method: 'signTransaction',
        tx: txHash,
        wallet: walletAddress,
        callback: callbackUrl,
      };

      const encodedRequest = encodeRequest(request);
      const botUrl = `https://t.me/devReptileBot?startapp=${encodedRequest}`;

      if (typeof window !== 'undefined') {
        window.open(botUrl, '_blank');
      }
    },
    [encodeRequest],
  );

  const handleCallback = useCallback(
    (encodedResult: string): ConnectResult | SignTransactionResult => {
      return decodeResult(encodedResult);
    },
    [decodeResult],
  );

  const getWalletAddress = useCallback((): string | null => {
    return account?.address || null;
  }, [account]);

  const isConnected = useCallback((): boolean => {
    return !!account;
  }, [account]);

  return {
    connect,
    signTransaction,
    handleCallback,
    getWalletAddress,
    isConnected,
    account,
  };
};

export default useVaranWallet;
