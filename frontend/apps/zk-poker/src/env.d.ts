interface ImportMetaEnv {
  readonly VITE_NODE_ADDRESS: string;
  readonly VITE_FT_ADDRESS: string;
  readonly VITE_TESTNET_WEBSITE_ADDRESS: string;
  readonly VITE_GASLESS_BACKEND_ADDRESS: string;
  readonly VITE_VOUCHER_LIMIT: string;
  readonly VITE_SIGNLESS_DEFAULT_DURATION_MINUTES: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
