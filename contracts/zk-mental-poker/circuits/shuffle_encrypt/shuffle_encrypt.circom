pragma circom 2.1.6;

include "../common/elgamal.circom";
include "../common/matrix.circom";
include "../common/permutation.circom";
include "../common/babyjubjub.circom";
include "../node_modules/circomlib/circuits/bitify.circom";
include "shuffle_encrypt_template.circom";

// Main encryption circuit over Bandersnatch
// Inputs: 52 cards, each card is (ic0 + ic1) = 2 points = 6 scalars
// Outputs: isValid == 1 if permutation and encryption match

template ShuffleEncrypt(numCards) {
    var numBits = 64;
    // Bandersnatch base point coordinates (x, y)
    var baseX = 0x29c132cc2c0b34c5743711777bbe42f32b79c022ad998465e1e71866a252ae18;
    var baseY = 0x2a6c669eda123e0f157d8b50badcd586358cad81eee464605e3167b6cc974166;

    signal input pk[3];                     // public key in projective form (X, Y, Z)
    signal input original[6][numCards];     // original matrix: [c0.X, c0.Y, c0.Z, c1.X, c1.Y, c1.Z]
    signal input permuted[6][numCards];     // shuffled+encrypted matrix
    signal input R[numCards];               // random scalars r_i
    signal output isValid;                  // output

    component encryptor = ShuffleEncryptTemplateV2(baseX, baseY, numCards, numBits);
    for (var i = 0; i < 3; i++) {
        encryptor.pk[i] <== pk[i];
    }
    for (var i = 0; i < numCards; i++) {
        encryptor.R[i] <== R[i];
        for (var j = 0; j < 6; j++) {
            encryptor.original[j][i] <== original[j][i];
            encryptor.permuted[j][i] <== permuted[j][i];
        }
    }

    isValid <== encryptor.isValid;
}

component main {public [pk, original, permuted]} = ShuffleEncrypt(52);
