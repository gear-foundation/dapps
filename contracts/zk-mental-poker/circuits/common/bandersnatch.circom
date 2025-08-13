pragma circom 2.1.6;

template EcCheck() {
    signal input x;
    signal input y;

    // -5x^2 + y^2 = 1 + d*x^2*y^2
    var a = -5;
    var d = 45022363124591815672509500913686876175488063829319466900776701791074614335719;

    signal x2 <== x * x;
    signal y2 <== y * y;
    signal lhs <== a * x2 + y2;
    signal rhs <== 1 + d * x2 * y2;

    lhs - rhs === 0;
}

component main = EcCheck();
