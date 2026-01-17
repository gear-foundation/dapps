pragma circom 2.1.6;

include "../node_modules/circomlib/circuits/bitify.circom";
include "../node_modules/circomlib/circuits/compconstant.circom";

template ecDecompress() {
    signal input x;         // base field elements of inner curve
    signal input s;         // boolean selector
    signal input delta;     // base field elements of inner curve
    signal output y;        // base field elements of inner curve
    signal x_square;
    signal delta_square;
    signal tmp[2];

    component n2b = Num2Bits(254);
    n2b.in <== delta;
    // On Baby JubJub curve, q = 21888242871839275222246405745257275088548364400416034343698204186575808495617
    // (q-1)/2 = 10944121435919637611123202872628637544274182200208017171849102093287904247808;
    component cmp = CompConstant(10944121435919637611123202872628637544274182200208017171849102093287904247808);
    for (var i = 0; i < 254; i++) {
        cmp.in[i] <== n2b.out[i];
    }
    cmp.out === 0;

    x_square <== x * x;
    delta_square <== delta * delta;
    // Check if the point is on baby jubjub curve: 168700*x^2 + y^2 = 1 + 168696*x^2*y^2
    // https://github.com/iden3/circomlibjs/blob/main/src/babyjub.js#L85-L95
    168700*x_square + delta_square === 1 + 168696 * x_square * delta_square;
    
    tmp[0] <== s*delta;
    tmp[1] <== (s-1) * delta;
    y <== tmp[0] + tmp[1];
}

