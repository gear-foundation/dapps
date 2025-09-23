pragma circom 2.1.6;

include "../common/elgamal.circom";

template MainTestDecrypt() {
    var numBits = 64;

    signal input c0[3];
    signal input sk;
    signal input expected[3]; 

    signal output isValid;

    component decrypt = ElGamalDecrypt(numBits);

    decrypt.c0[0] <== c0[0];
    decrypt.c0[1] <== c0[1];
    decrypt.c0[2] <== c0[2];
    decrypt.sk <== sk;
   
    decrypt.expected[0] <== expected[0];
    decrypt.expected[1] <== expected[1];
    decrypt.expected[2] <== expected[2];
   
    isValid <== decrypt.isValid;
    isValid === 1;
}

component main = MainTestDecrypt();
