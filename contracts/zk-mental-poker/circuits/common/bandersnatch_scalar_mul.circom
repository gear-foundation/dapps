pragma circom 2.1.6;

include "../node_modules/circomlib/circuits/bitify.circom";
include "bandersnatch_add.circom";
include "bandersnatch_double.circom";

template BandersnatchScalarMulProjective(nBits) {
    signal input X;
    signal input Y;
    signal input Z;
    signal input scalar;

    signal output Xout;
    signal output Yout;
    signal output Zout;

    // Проверка нейтрального элемента (0, 1, 1)
    component isX0 = IsZero();
    component isY1 = IsEqual();
    component isZ1 = IsEqual();

    isX0.in <== X;
    isY1.in[0] <== Y; isY1.in[1] <== 1;
    isZ1.in[0] <== Z; isZ1.in[1] <== 1;

    signal tmpNeutral;
    signal isNeutral;
    tmpNeutral <== isX0.out * isY1.out;
    isNeutral <== tmpNeutral * isZ1.out;

    // Подмена нейтрального элемента на безопасное значение
    signal nonNeutralX <== isNeutral * 0 + (1 - isNeutral) * X;
    signal nonNeutralY <== isNeutral * 1 + (1 - isNeutral) * Y;
    signal nonNeutralZ <== isNeutral * 1 + (1 - isNeutral) * Z;

    // Преобразование скаляра в биты (MSB first)
    component bits = Num2Bits(nBits);
    bits.in <== scalar;

    // Аккумулятор результата
    signal accX[nBits + 1];
    signal accY[nBits + 1];
    signal accZ[nBits + 1];

    // Инициализация аккумулятора точкой P (если первый бит 1) 
    // или нейтральным элементом (если первый бит 0)
    accX[0] <== bits.out[nBits-1] * nonNeutralX;
    accY[0] <== bits.out[nBits-1] * nonNeutralY + (1 - bits.out[nBits-1]) * 1;
    accZ[0] <== bits.out[nBits-1] * nonNeutralZ + (1 - bits.out[nBits-1]) * 1;

    component dbls[nBits-1];
    component adds[nBits-1];

    // Начинаем со второго бита (индекс nBits-2), так как первый бит уже обработан
    for (var i = 0; i < nBits-1; i++) {
        var bitIndex = nBits - 2 - i;
        
        // Удвоение аккумулятора
        dbls[i] = BandersnatchDoubleProjective();
        dbls[i].X <== accX[i];
        dbls[i].Y <== accY[i];
        dbls[i].Z <== accZ[i];
        
        // Условное добавление точки P если текущий бит равен 1
        adds[i] = BandersnatchAddProjective();
        adds[i].X1 <== dbls[i].X3;
        adds[i].Y1 <== dbls[i].Y3;
        adds[i].Z1 <== dbls[i].Z3;
        adds[i].X2 <== nonNeutralX;
        adds[i].Y2 <== nonNeutralY;
        adds[i].Z2 <== nonNeutralZ;
        
        // Выбор результата в зависимости от бита
        accX[i+1] <== dbls[i].X3 + bits.out[bitIndex] * (adds[i].X3 - dbls[i].X3);
        accY[i+1] <== dbls[i].Y3 + bits.out[bitIndex] * (adds[i].Y3 - dbls[i].Y3);
        accZ[i+1] <== dbls[i].Z3 + bits.out[bitIndex] * (adds[i].Z3 - dbls[i].Z3);
    }

    // Особый случай: если все биты 0, результат должен быть нейтральным элементом
    component isZeroScalar = IsZero();
    isZeroScalar.in <== scalar;
    
    // Финальный результат с учетом особого случая
    Xout <== (1 - isZeroScalar.out) * accX[nBits-1];
    Yout <== isZeroScalar.out + (1 - isZeroScalar.out) * accY[nBits-1];
    Zout <== isZeroScalar.out + (1 - isZeroScalar.out) * accZ[nBits-1];
}