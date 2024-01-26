/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_NODE_ADDRESS: string;
  readonly VITE_CONTRACT_ADDRESS: `0x${string}`;
  readonly VITE_GASLESS_BACKEND_ADDRESS: string;
}

interface ImportMeta {
  env: ImportMetaEnv;
}
