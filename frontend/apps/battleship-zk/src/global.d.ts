import { FC, ReactNode, SVGProps } from 'react';

// declaring .wasm, since TS doesn't support experimental modules
// source: https://github.com/microsoft/TypeScript/issues/31713
declare module '*.wasm' {
  const value: string;
  export default value;
}

declare module '*.txt' {
  const value: string;
  export default value;
}

declare global {
  type BaseComponentProps = {
    children?: ReactNode;
    className?: string;
  };

  type PickPartial<T, K extends keyof T> = T | Pick<T, K>;

  // in case Object.entries return value is immutable
  // ref: https://stackoverflow.com/a/60142095
  type Entries<T> = {
    [K in keyof T]: [K, T[K]];
  }[keyof T][];

  type SVGComponent = FC<
    SVGProps<SVGSVGElement> & {
      title?: string | undefined;
    }
  >;
}

// Miscellaneous types.
declare global {
  interface Window {
    // Nova Wallet will have this window property.
    // eslint-disable-next-line
    walletExtension?: any;
  }
}
