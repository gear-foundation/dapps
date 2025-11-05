interface ImportMetaEnv {
  readonly VITE_NODE_ADDRESS: string;
  readonly VITE_IPFS_GATEWAY_ADDRESS: string;
  readonly VITE_DNS_API_URL: string;
  readonly VITE_DNS_NAME: string;
  readonly VITE_NFT_EXPLORER_URL: string;
  readonly VITE_SENTRY_DSN: string | undefined;
  readonly VITE_AUTH_API_ADDRESS: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
