pragma circom 2.1.6;

include "../node_modules/circomlib/circuits/comparators.circom";

template BandersnatchAddProjective() {
    signal input X1;
    signal input Y1;
    signal input Z1;

    signal input X2;
    signal input Y2;
    signal input Z2;

    signal output X3;
    signal output Y3;
    signal output Z3;

    var a = -5;
    var d = 45022363124591815672509500913686876175488063829319466900776701791074614335719;

    // Проверка нейтральных точек (0, 1, 1)
    component isX1Zero = IsZero(); isX1Zero.in <== X1;
    component isY1One = IsEqual(); isY1One.in[0] <== Y1; isY1One.in[1] <== 1;
    component isZ1One = IsEqual(); isZ1One.in[0] <== Z1; isZ1One.in[1] <== 1;

    component isX2Zero = IsZero(); isX2Zero.in <== X2;
    component isY2One = IsEqual(); isY2One.in[0] <== Y2; isY2One.in[1] <== 1;
    component isZ2One = IsEqual(); isZ2One.in[0] <== Z2; isZ2One.in[1] <== 1;

    signal tmp1 <== isX1Zero.out * isY1One.out;
    signal isP1Neutral <== tmp1 * isZ1One.out;

    signal tmp2 <== isX2Zero.out * isY2One.out;
    signal isP2Neutral <== tmp2 * isZ2One.out;

    // Основная формула сложения как в Python-коде, но с квадратичными ограничениями
    // A = z*zz
    signal A <== Z1 * Z2;
    
    // B = A**2
    signal B <== A * A;
    
    // C = x*xx
    signal C <== X1 * X2;
    
    // D = y*yy
    signal D <== Y1 * Y2;
    
    // E = self.curve.d*C*D
    // Разбиваем на два шага для соблюдения квадратичности
    signal dC <== d * C;
    signal E <== dC * D;
    
    // F = B-E
    signal F <== B - E;
    
    // G = B+E
    signal G <== B + E;
    
    // X = A*F*((x+y) * (xx+yy) - C - D)
    // Разбиваем вычисление на последовательность квадратичных операций
    signal X1plusY1 <== X1 + Y1;
    signal X2plusY2 <== X2 + Y2;
    signal multSum <== X1plusY1 * X2plusY2;
    signal CplusD <== C + D;
    signal coreTerm <== multSum - CplusD;
    
    // Разбиваем тройное произведение A*F*coreTerm на два шага
    signal AF <== A * F;
    signal coreX3 <== AF * coreTerm;
    
    // Y = A*G*(D-self.curve.a*C)
    // Разбиваем на квадратичные операции
    signal aTimesC <== a * C;
    signal DminusAC <== D - aTimesC;
    
    // Разбиваем тройное произведение A*G*DminusAC на два шага
    signal AG <== A * G;
    signal coreY3 <== AG * DminusAC;
    
    // Z = F*G
    signal coreZ3 <== F * G;

    // Альтернативный результат если одна точка нейтральная
    signal Xalt_p1 <== isP1Neutral * X2;
    signal Xalt_p2 <== isP2Neutral * X1;
    signal Xalt <== Xalt_p1 + Xalt_p2;

    signal Yalt_p1 <== isP1Neutral * Y2;
    signal Yalt_p2 <== isP2Neutral * Y1;
    signal Yalt <== Yalt_p1 + Yalt_p2;

    signal Zalt_p1 <== isP1Neutral * Z2;
    signal Zalt_p2 <== isP2Neutral * Z1;
    signal Zalt <== Zalt_p1 + Zalt_p2;

    signal tmp_or <== isP1Neutral + isP2Neutral;
    signal isEitherNeutral <== tmp_or - isP1Neutral * isP2Neutral;

    // Финальный выбор результата
    signal finalX <== (1 - isEitherNeutral) * coreX3;
    signal finalY <== (1 - isEitherNeutral) * coreY3;
    signal finalZ <== (1 - isEitherNeutral) * coreZ3;

    X3 <== finalX + isEitherNeutral * Xalt;
    Y3 <== finalY + isEitherNeutral * Yalt;
    Z3 <== finalZ + isEitherNeutral * Zalt;
}