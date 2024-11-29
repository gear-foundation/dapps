export {};

declare global {
  interface Window {
    walletExtension: { isNovaWallet: boolean };
  }
}
