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

declare module '*.svg' {
  import type { FunctionComponent, SVGProps } from 'react';
  export const ReactComponent: FunctionComponent<SVGProps<SVGSVGElement>>;
  const src: string;
  export default src;
}
