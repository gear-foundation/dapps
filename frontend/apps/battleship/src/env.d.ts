interface ImportMetaEnv {
  readonly VITE_NODE_ADDRESS: string;
  readonly VITE_FT_ADDRESS: string;
  readonly VITE_TESTNET_WEBSITE_ADDRESS: string;
  readonly VITE_DNS_API_URL: string;
  readonly VITE_DNS_NAME: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
