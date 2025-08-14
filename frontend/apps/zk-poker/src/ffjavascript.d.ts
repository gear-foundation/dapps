declare module 'ffjavascript' {
  interface F1FieldInstance {
    // Arithmetic operations
    add: (a: bigint, b: bigint) => bigint;
    sub: (a: bigint, b: bigint) => bigint;
    mul: (a: bigint, b: bigint) => bigint;
    div: (a: bigint, b: bigint) => bigint;
    square: (x: bigint) => bigint;
    neg: (x: bigint) => bigint;
    inv: (x: bigint) => bigint;

    // Comparison operations
    eq: (a: bigint, b: bigint) => boolean;
  }

  interface F1FieldConstructor {
    new (modulus: bigint): F1FieldInstance;
  }

  export const F1Field: F1FieldConstructor;
}
