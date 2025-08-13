pragma circom 2.1.6;

include "../node_modules/circomlib/circuits/comparators.circom";

template BandersnatchDoubleProjective() {
    signal input X;
    signal input Y;
    signal input Z;

    signal output X3;
    signal output Y3;
    signal output Z3;

    var a = -5;

    // Проверка на нейтральную точку (0,1,1)
    component isX0 = IsZero();
    component isY1 = IsEqual();
    component isZ1 = IsEqual();

    isX0.in <== X;
    isY1.in[0] <== Y;
    isY1.in[1] <== 1;
    isZ1.in[0] <== Z;
    isZ1.in[1] <== 1;

    signal tmp;
    signal isNeutral;
    tmp <== isX0.out * isY1.out;
    isNeutral <== tmp * isZ1.out;

    // Стандартные вычисления удвоения
    signal B;
    signal C;
    signal D;
    signal E;
    signal F;
    signal H;
    signal J;
    signal rawX3;
    signal rawY3;
    signal rawZ3;

    B <== (X + Y) * (X + Y);
    C <== X * X;
    D <== Y * Y;
    E <== a * C;
    F <== E + D;
    H <== Z * Z;
    J <== F - 2 * H;

    rawX3 <== (B - C - D) * J;
    rawY3 <== F * (E - D);
    rawZ3 <== F * J;

    // Если isNeutral == 1 → вернуть (0, 1, 1), иначе (rawX3, rawY3, rawZ3)
    X3 <== isNeutral * 0 + (1 - isNeutral) * rawX3;
    Y3 <== isNeutral * 1 + (1 - isNeutral) * rawY3;
    Z3 <== isNeutral * 1 + (1 - isNeutral) * rawZ3;
}
