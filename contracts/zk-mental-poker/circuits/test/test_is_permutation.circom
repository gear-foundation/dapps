pragma circom 2.1.6;

include "../common/permutation.circom";

template Main(n) {
    signal input in[n];
    signal output out;

    component check = IsPermutation(n);
    for (var i = 0; i < n; i++) {
        check.in[i] <== in[i];
    }
    out <== check.out;
}

component main = Main(52);
