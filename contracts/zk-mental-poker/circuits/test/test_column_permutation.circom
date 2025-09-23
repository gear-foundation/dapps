pragma circom 2.1.6;

include "../common/permutation.circom";

template Main(rows, cols) {
    signal input original[rows][cols];
    signal input permuted[rows][cols];
    signal input permutation[cols]; 
    signal output isValid;

    component check = ColumnPermutationCheck(rows, cols);
    for (var i = 0; i < rows; i++) {
        for (var j = 0; j < cols; j++) {
            check.original[i][j] <== original[i][j];
            check.permuted[i][j] <== permuted[i][j];
        }
    }
    for (var j = 0; j < cols; j++) {
        check.permutation[j] <== permutation[j];
    }
    isValid <== check.isValid;
}

component main = Main(6, 52);
