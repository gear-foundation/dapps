interface ImportMetaEnv {
  readonly VITE_NODE_ADDRESS: string;
  readonly VITE_FT_ADDRESS: string;
  readonly VITE_TESTNET_WEBSITE_ADDRESS: string;
  readonly VITE_GASLESS_BACKEND_ADDRESS: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
