pragma circom 2.1.6;

/// Checks that `in` is a boolean.
template Boolean() {
    signal input in;
    in * (in -1) === 0;
}