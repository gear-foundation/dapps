// declaring .wasm, since TS doesn't support experimental modules
// source: https://github.com/microsoft/TypeScript/issues/31713

declare module "*.wasm" {
  const value: string;
  export default value;
}

declare module "*.txt" {
  const value: string;
  export default value;
}

interface ImportMetaEnv {
  readonly VITE_STORE_ADDRESS: string;
  readonly VITE_FT_ADDRESS: string;
  readonly VITE_BATTLE_ADDRESS: string;
  readonly VITE_NODE_ADDRESS: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
