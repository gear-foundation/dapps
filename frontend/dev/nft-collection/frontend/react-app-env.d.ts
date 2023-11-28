/// <reference types="react-scripts" />

declare module '*.scss' {
  const css: { [key: string]: string };
  export default css;
}
declare module '*.sass' {
  const css: { [key: string]: string };
  export default css;
}

declare module '*.wasm' {
  const value: string;
  export default value;
}

declare module '*.txt' {
  const value: string;
  export default value;
}
