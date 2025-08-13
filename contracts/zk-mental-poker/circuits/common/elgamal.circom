/*
 * Note:
 * sk = sk_A + sk_B + sk_C
 * pk = sk*g

 * Init:
 * (0, m)
 * Alice Encrypt:
 * (a*g, m + a*pk)
 * Bob Encrypt:
 * ((a+b)*g, m + (a+b)*pk)
 * Charlie Encrypt:
 * ((a+b+c)*g, m + (a+b+c)*pk)
 * Bob Decrypt:
 * m+(a+b+c)*pk - sk_B*(a+b+c)*g
 * ...
*/

pragma circom 2.1.6;

include "../node_modules/circomlib/circuits/bitify.circom";
include "../node_modules/circomlib/circuits/escalarmulfix.circom";
include "../node_modules/circomlib/circuits/escalarmulany.circom";
include "bandersnatch_scalar_mul.circom";
include "bandersnatch_add.circom";

template IdentityPoint() {
    signal output X;
    signal output Y;
    signal output Z;

    X <== 0;
    Y <== 1;
    Z <== 1;
}

template BatchElGamalVerifyNoAlpha(numBits, baseX, baseY, numCards) {
    signal input original[6][numCards];
    signal input encrypted[6][numCards];
    signal input R[numCards];
    signal input pk[3];
    signal output isValid;

    component identity = IdentityPoint();

    component X_sum[numCards];
    component Y_sum[numCards];
    component IC0_sum[numCards];
    component IC1_sum[numCards];

    // First card initialization
    X_sum[0] = BandersnatchAddProjective();
    X_sum[0].X1 <== encrypted[0][0];
    X_sum[0].Y1 <== encrypted[1][0];
    X_sum[0].Z1 <== encrypted[2][0];
    X_sum[0].X2 <== identity.X;
    X_sum[0].Y2 <== identity.Y;
    X_sum[0].Z2 <== identity.Z;

    Y_sum[0] = BandersnatchAddProjective();
    Y_sum[0].X1 <== encrypted[3][0];
    Y_sum[0].Y1 <== encrypted[4][0];
    Y_sum[0].Z1 <== encrypted[5][0];
    Y_sum[0].X2 <== identity.X;
    Y_sum[0].Y2 <== identity.Y;
    Y_sum[0].Z2 <== identity.Z;

    IC0_sum[0] = BandersnatchAddProjective();
    IC0_sum[0].X1 <== original[0][0];
    IC0_sum[0].Y1 <== original[1][0];
    IC0_sum[0].Z1 <== original[2][0];
    IC0_sum[0].X2 <== identity.X;
    IC0_sum[0].Y2 <== identity.Y;
    IC0_sum[0].Z2 <== identity.Z;

    IC1_sum[0] = BandersnatchAddProjective();
    IC1_sum[0].X1 <== original[3][0];
    IC1_sum[0].Y1 <== original[4][0];
    IC1_sum[0].Z1 <== original[5][0];
    IC1_sum[0].X2 <== identity.X;
    IC1_sum[0].Y2 <== identity.Y;
    IC1_sum[0].Z2 <== identity.Z;

    signal sumR[numCards];  
    sumR[0] <== R[0];

    // Subsequent summation
    for (var i = 1; i < numCards; i++) {
        X_sum[i] = BandersnatchAddProjective();
        X_sum[i].X1 <== X_sum[i-1].X3;
        X_sum[i].Y1 <== X_sum[i-1].Y3;
        X_sum[i].Z1 <== X_sum[i-1].Z3;
        X_sum[i].X2 <== encrypted[0][i];
        X_sum[i].Y2 <== encrypted[1][i];
        X_sum[i].Z2 <== encrypted[2][i];

        Y_sum[i] = BandersnatchAddProjective();
        Y_sum[i].X1 <== Y_sum[i-1].X3;
        Y_sum[i].Y1 <== Y_sum[i-1].Y3;
        Y_sum[i].Z1 <== Y_sum[i-1].Z3;
        Y_sum[i].X2 <== encrypted[3][i];
        Y_sum[i].Y2 <== encrypted[4][i];
        Y_sum[i].Z2 <== encrypted[5][i];

        IC0_sum[i] = BandersnatchAddProjective();
        IC0_sum[i].X1 <== IC0_sum[i-1].X3;
        IC0_sum[i].Y1 <== IC0_sum[i-1].Y3;
        IC0_sum[i].Z1 <== IC0_sum[i-1].Z3;
        IC0_sum[i].X2 <== original[0][i];
        IC0_sum[i].Y2 <== original[1][i];
        IC0_sum[i].Z2 <== original[2][i];

        IC1_sum[i] = BandersnatchAddProjective();
        IC1_sum[i].X1 <== IC1_sum[i-1].X3;
        IC1_sum[i].Y1 <== IC1_sum[i-1].Y3;
        IC1_sum[i].Z1 <== IC1_sum[i-1].Z3;
        IC1_sum[i].X2 <== original[3][i];
        IC1_sum[i].Y2 <== original[4][i];
        IC1_sum[i].Z2 <== original[5][i];

        sumR[i] <== sumR[i - 1] + R[i];
    }

    signal R_sum;
    R_sum <== sumR[numCards - 1];

    // Compute r*G
    component RG = BandersnatchScalarMulProjective(numBits);
    RG.X <== baseX;
    RG.Y <== baseY;
    RG.Z <== 1;
    RG.scalar <== R_sum;

    // Compute r*pk
    component Rpk = BandersnatchScalarMulProjective(numBits);
    Rpk.X <== pk[0];
    Rpk.Y <== pk[1];
    Rpk.Z <== pk[2];
    Rpk.scalar <== R_sum;

    // Add RG + IC0_sum 
    component RG_plus_IC0 = BandersnatchAddProjective();
    RG_plus_IC0.X1 <== RG.Xout;
    RG_plus_IC0.Y1 <== RG.Yout;
    RG_plus_IC0.Z1 <== RG.Zout;
    RG_plus_IC0.X2 <== IC0_sum[numCards-1].X3;
    RG_plus_IC0.Y2 <== IC0_sum[numCards-1].Y3;
    RG_plus_IC0.Z2 <== IC0_sum[numCards-1].Z3;

    // Add Rpk + IC1_sum 
    component Rpk_plus_IC1 = BandersnatchAddProjective();
    Rpk_plus_IC1.X1 <== Rpk.Xout;
    Rpk_plus_IC1.Y1 <== Rpk.Yout;
    Rpk_plus_IC1.Z1 <== Rpk.Zout;
    Rpk_plus_IC1.X2 <== IC1_sum[numCards-1].X3;
    Rpk_plus_IC1.Y2 <== IC1_sum[numCards-1].Y3;
    Rpk_plus_IC1.Z2 <== IC1_sum[numCards-1].Z3;

    // Now check if X_sum == RG + IC0_sum
    component checkRG = IsEqualProjective();
    checkRG.X1 <== X_sum[numCards-1].X3;
    checkRG.Y1 <== X_sum[numCards-1].Y3;
    checkRG.Z1 <== X_sum[numCards-1].Z3;
    checkRG.X2 <== RG_plus_IC0.X3;
    checkRG.Y2 <== RG_plus_IC0.Y3;
    checkRG.Z2 <== RG_plus_IC0.Z3;

    // Check if Y_sum == Rpk + IC1_sum
    component checkRpk = IsEqualProjective();
    checkRpk.X1 <== Y_sum[numCards-1].X3;
    checkRpk.Y1 <== Y_sum[numCards-1].Y3;
    checkRpk.Z1 <== Y_sum[numCards-1].Z3;
    checkRpk.X2 <== Rpk_plus_IC1.X3;
    checkRpk.Y2 <== Rpk_plus_IC1.Y3;
    checkRpk.Z2 <== Rpk_plus_IC1.Z3;

    isValid <== checkRG.isEqual * checkRpk.isEqual;
}

template IsEqualProjective() {
    signal input X1;
    signal input Y1;
    signal input Z1;
    signal input X2;
    signal input Y2;
    signal input Z2;

    signal output isEqual;

    // Compute cross-multiplications
    signal X1Z2;
    signal X2Z1;
    signal Y1Z2;
    signal Y2Z1;

    X1Z2 <== X1 * Z2;
    X2Z1 <== X2 * Z1;
    Y1Z2 <== Y1 * Z2;
    Y2Z1 <== Y2 * Z1;

    // Compare results
    component xEq = IsEqual();
    xEq.in[0] <== X1Z2;
    xEq.in[1] <== X2Z1;

    component yEq = IsEqual();
    yEq.in[0] <== Y1Z2;
    yEq.in[1] <== Y2Z1;

    isEqual <== xEq.out * yEq.out;
}

// ElGamalEncrypt:
// c0 = r * g + ic0
// c1 = r * pk + ic1
template ElGamalEncrypt(numBits, baseX, baseY) {
    signal input ic0[3];  // Projective point (X, Y, Z)
    signal input ic1[3];  // Projective point (X, Y, Z)
    signal input r;       // Random scalar
    signal input pk[3];   // Public key (X, Y, Z)
    signal output c0[3];  // Encrypted output 1
    signal output c1[3];  // Encrypted output 2


    // c0 = r * g + ic0
    component bitDecomposition = Num2Bits(numBits);
    bitDecomposition.in <== r;
    component computeC0 = BandersnatchScalarMulProjective(numBits);
    computeC0.X <== baseX;
    computeC0.Y <== baseY;
    computeC0.Z <== 1;
    computeC0.scalar <== r;
   
    component adder0 = BandersnatchAddProjective();
    adder0.X1 <== computeC0.Xout;
    adder0.Y1 <== computeC0.Yout;
    adder0.Z1 <== computeC0.Zout;
    adder0.X2 <== ic0[0];
    adder0.Y2 <== ic0[1];
    adder0.Z2 <== ic0[2];
    c0[0] <== adder0.X3;
    c0[1] <== adder0.Y3;
    c0[2] <== adder0.Z3;

    // c1 = r * pk + ic1
    component computeC1 = BandersnatchScalarMulProjective(numBits);
    computeC1.X <== pk[0];
    computeC1.Y <== pk[1];
    computeC1.Z <== pk[2];
    computeC1.scalar <== r;
    
    component adder1 = BandersnatchAddProjective();
    adder1.X1 <== computeC1.Xout;
    adder1.Y1 <== computeC1.Yout;
    adder1.Z1 <== computeC1.Zout;
    adder1.X2 <== ic1[0];
    adder1.Y2 <== ic1[1];
    adder1.Z2 <== ic1[2];
    c1[0] <== adder1.X3;
    c1[1] <== adder1.Y3;
    c1[2] <== adder1.Z3;
}

// ElGamalDecrypt:
//  - sk * c0

template ElGamalDecrypt(numBits) {
    signal input c0[3];  // c0 of ElGamalEncrypt
    signal input sk;     // secret key, {0, 1}^numBits
    signal input expected[3]; 

    signal output isValid;

    component scalarMul = BandersnatchScalarMulProjective(numBits);
    scalarMul.X <== c0[0];
    scalarMul.Y <== c0[1];
    scalarMul.Z <== c0[2];
    scalarMul.scalar <== sk;

    component check = IsEqualProjective();
    check.X1 <== expected[0];
    check.Y1 <== expected[1];
    check.Z1 <== expected[2];
    check.X2 <== 0 - scalarMul.Xout;
    check.Y2 <== scalarMul.Yout;
    check.Z2 <== scalarMul.Zout;
    isValid <== check.isEqual;
}