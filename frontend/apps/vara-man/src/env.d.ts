// declaring .wasm, since TS doesn't support experimental modules
// source: https://github.com/microsoft/TypeScript/issues/31713

import { HexString } from '@polkadot/util/types';

declare module '*.wasm' {
  const value: string;
  export default value;
}

declare module '*.txt' {
  const value: string;
  export default value;
}

interface ImportMetaEnv {
  readonly VITE_CONTRACT_ADDRESS: string;
  readonly VITE_NODE_ADDRESS: HexString;
  readonly VITE_DNS_API_URL: string;
  readonly VITE_DNS_NAME: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
