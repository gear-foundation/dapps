interface ImportMetaEnv {
  readonly VITE_NODE_ADDRESS: string;
  readonly VITE_AUTH_API_ADDRESS: string;
  readonly VITE_CONTRACT_ADDRESS: string;
  readonly VITE_DEFAULT_NODES_URL: string;
  readonly VITE_STAGING_NODES_URL: string;
  readonly VITE_GTM_ID_TTT: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
