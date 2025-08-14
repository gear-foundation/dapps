/// <reference types="vite/client" />

declare module '*.idl' {
  const value: string;
  export default value;
}

declare type Poseidon = ((args: (number | string)[]) => number) & {
  F: {
    toString: (value: number) => string;
  };
};

declare module '*poseidon' {
  const buildPoseidon: () => Promise<Poseidon>;
}
