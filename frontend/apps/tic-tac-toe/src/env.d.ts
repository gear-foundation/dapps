interface ImportMetaEnv {
  readonly VITE_NODE_ADDRESS: string;
  readonly VITE_GASLESS_BACKEND_ADDRESS: string;
  readonly VITE_DEFAULT_NODES_URL: string;
  readonly VITE_DNS_API_URL: string;
  readonly VITE_DNS_NAME: string;
  readonly VITE_STAGING_NODES_URL: string;
  readonly VITE_SENTRY_DSN_TTT: string;
  readonly VITE_VOUCHER_LIMIT: string;
  readonly VITE_GTM_ID_TTT: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
