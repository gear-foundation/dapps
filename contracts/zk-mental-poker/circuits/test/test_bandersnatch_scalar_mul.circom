pragma circom 2.1.6;

include "../common/bandersnatch_scalar_mul.circom";

template Main() {
    signal input X;
    signal input Y;
    signal input Z;
    signal input scalar;

    signal output Xout;
    signal output Yout;
    signal output Zout;

    component mul = BandersnatchScalarMulProjective(128); 
    mul.X <== X;
    mul.Y <== Y;
    mul.Z <== Z;
    mul.scalar <== scalar;

    Xout <== mul.Xout;
    Yout <== mul.Yout;
    Zout <== mul.Zout;
}

component main = Main();