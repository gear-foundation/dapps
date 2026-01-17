pragma circom 2.1.6;

include "../common/bandersnatch_double.circom";

template Main() {
    signal input X;
    signal input Y;
    signal input Z;

    signal output X3;
    signal output Y3;
    signal output Z3;

    component double = BandersnatchDoubleProjective();
    double.X <== X;
    double.Y <== Y;
    double.Z <== Z;

    X3 <== double.X3;
    Y3 <== double.Y3;
    Z3 <== double.Z3;
}

component main = Main();
