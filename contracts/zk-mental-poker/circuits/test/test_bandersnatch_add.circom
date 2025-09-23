pragma circom 2.1.6;

include "../common/bandersnatch_add.circom";

template Main() {
    signal input X1;
    signal input Y1;
    signal input Z1;

    signal input X2;
    signal input Y2;
    signal input Z2;

    signal output X3;
    signal output Y3;
    signal output Z3;

    component add = BandersnatchAddProjective();
    add.X1 <== X1;
    add.Y1 <== Y1;
    add.Z1 <== Z1;

    add.X2 <== X2;
    add.Y2 <== Y2;
    add.Z2 <== Z2;

    X3 <== add.X3;
    Y3 <== add.Y3;
    Z3 <== add.Z3;
}

component main = Main();
