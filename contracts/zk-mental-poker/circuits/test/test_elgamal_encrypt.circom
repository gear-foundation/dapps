pragma circom 2.1.6;

include "../common/elgamal.circom";

template MainTestEncrypt() {
    var numBits = 128;

    var baseX = 0x29c132cc2c0b34c5743711777bbe42f32b79c022ad998465e1e71866a252ae18;
    var baseY = 0x2a6c669eda123e0f157d8b50badcd586358cad81eee464605e3167b6cc974166;

    signal input ic0[3];
    signal input ic1[3];
    signal input r;
    signal input pk[3];
    signal output c0[3];
    signal output c1[3];

    component encrypt = ElGamalEncrypt(numBits, baseX, baseY);

    encrypt.ic0[0] <== ic0[0];
    encrypt.ic0[1] <== ic0[1];
    encrypt.ic0[2] <== ic0[2];
    encrypt.ic1[0] <== ic1[0];
    encrypt.ic1[1] <== ic1[1];
    encrypt.ic1[2] <== ic1[2];
    encrypt.r <== r;
    encrypt.pk[0] <== pk[0];
    encrypt.pk[1] <== pk[1];
    encrypt.pk[2] <== pk[2];

    c0[0] <== encrypt.c0[0];
    c0[1] <== encrypt.c0[1];
    c0[2] <== encrypt.c0[2];
    c1[0] <== encrypt.c1[0];
    c1[1] <== encrypt.c1[1];
    c1[2] <== encrypt.c1[2];
}

component main = MainTestEncrypt();
