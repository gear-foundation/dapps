interface ImportMetaEnv {
  readonly VITE_NODE_ADDRESS: string;
  readonly VITE_GASLESS_BACKEND_ADDRESS: string;
  readonly VITE_DNS_API_URL: string;
  readonly VITE_DNS_NAME: string;
  readonly VITE_VOUCHER_LIMIT: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
