/// <reference types="vite/client" />

declare module '*.idl' {
  const value: string;
  export default value;
}

declare module '*poseidon' {
  const buildPoseidon: () => Poseidon;
}
