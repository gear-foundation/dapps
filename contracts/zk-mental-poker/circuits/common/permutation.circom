pragma circom 2.1.6;

// Checks whether a matrix's columns have been permuted according to a given permutation vector
// Inputs:
//   - original: [rows][cols] matrix
//   - permuted: [rows][cols] matrix
//   - permutation: array of column indices (length = cols)
// Output:
//   - isValid: 1 if columns are permuted correctly, else 0

include "../node_modules/circomlib/circuits/comparators.circom";
include "../common/permutation.circom";

template MultiMux1(n) {
    signal input c[n];      
    signal input s;         
    signal output out;

    component eqs[n];
    signal selected[n];
    signal partials[n];

    for (var i = 0; i < n; i++) {
        eqs[i] = IsEqual();
        eqs[i].in[0] <== s;
        eqs[i].in[1] <== i;

        selected[i] <== eqs[i].out;
        selected[i] * (selected[i] - 1) === 0;

        partials[i] <== c[i] * selected[i];
    }
    
    signal sumPartials[n+1];
    sumPartials[0] <== 0;
    for (var i = 0; i < n; i++) {
        sumPartials[i+1] <== sumPartials[i] + partials[i];
    }
    out <== sumPartials[n];
}

template ApplyPermutation(rows, cols) {
    signal input original[rows][cols];
    signal input permutation[cols];
    signal output permuted[rows][cols];

    component sels[cols][rows];

    for (var j = 0; j < cols; j++) {
        for (var i = 0; i < rows; i++) {
            sels[j][i] = MultiMux1(cols);
            for (var k = 0; k < cols; k++) {
                sels[j][i].c[k] <== original[i][k];
            }
            sels[j][i].s <== permutation[j];
            permuted[i][j] <== sels[j][i].out;
        }
    }
}

template ColumnPermutationCheck(rows, cols) {
    signal input original[rows][cols];
    signal input permuted[rows][cols];
    signal input permutation[cols];
    signal output isValid;

    signal checks[cols][rows];
    signal colCheck[cols];
    component sels[cols][rows];
    component eqChecks[cols][rows];
    
    // Объявляем массивы промежуточных продуктов перед циклами
    signal columnProduct[cols][rows+1];
    signal allColsProduct[cols+1];
    
    // Инициализируем начальные значения
    allColsProduct[0] <== 1;
    
    for (var j = 0; j < cols; j++) {
        // Инициализируем первый элемент продукта для текущего столбца
        columnProduct[j][0] <== 1;
        
        for (var i = 0; i < rows; i++) {
            // Используем MultiMux1 для выбора элемента из original по индексу permutation[j]
            sels[j][i] = MultiMux1(cols);
            for (var k = 0; k < cols; k++) {
                sels[j][i].c[k] <== original[i][k];
            }
            sels[j][i].s <== permutation[j];

            // Проверяем равенство выбранного элемента и элемента в permuted
            eqChecks[j][i] = IsEqual();
            eqChecks[j][i].in[0] <== sels[j][i].out;
            eqChecks[j][i].in[1] <== permuted[i][j];
            checks[j][i] <== eqChecks[j][i].out;

            // Накапливаем произведение проверок для текущего столбца
            columnProduct[j][i+1] <== columnProduct[j][i] * checks[j][i];
        }
        
        // Результат проверки для текущего столбца
        colCheck[j] <== columnProduct[j][rows];
        
        // Накапливаем произведение проверок по всем столбцам
        allColsProduct[j+1] <== allColsProduct[j] * colCheck[j];
    }

    // Проверяем валидность перестановки
    component permValidator = IsPermutation(cols);
    for (var i = 0; i < cols; i++) {
        permValidator.in[i] <== permutation[i];
    }

    // Финальный результат - произведение проверки перестановки и проверки всех столбцов
    isValid <== allColsProduct[cols] * permValidator.out;
}

template IsPermutation(n) {
    signal input in[n];
    signal output out;

    signal flags[n][n]; // flags[i][j] = 1 if in[i] == j
    signal sums[n];
    component eqs[n][n];
    component isOne[n];
    
    // Создаем промежуточные массивы для суммирования
    signal sumTemp[n][n+1];

    // Построение флагов
    for (var i = 0; i < n; i++) {
        for (var j = 0; j < n; j++) {
            eqs[i][j] = IsEqual();
            eqs[i][j].in[0] <== in[i];
            eqs[i][j].in[1] <== j;
            flags[i][j] <== eqs[i][j].out;
        }
    }

    // Подсчёт количества раз, которое j встречается
    for (var j = 0; j < n; j++) {
        // Инициализируем начальное значение суммы как 0
        sumTemp[j][0] <== 0;
        
        // Накапливаем сумму в промежуточном массиве
        for (var i = 0; i < n; i++) {
            sumTemp[j][i+1] <== sumTemp[j][i] + flags[i][j];
        }
        
        // Финальная сумма для j
        sums[j] <== sumTemp[j][n];
    }

    // Проверяем, что каждое число встречается ровно один раз
    // Используем промежуточный массив для накопления произведения
    signal outTemp[n+1];
    outTemp[0] <== 1;
    
    for (var j = 0; j < n; j++) {
        isOne[j] = IsEqual();
        isOne[j].in[0] <== sums[j];
        isOne[j].in[1] <== 1;
        
        outTemp[j+1] <== outTemp[j] * isOne[j].out;
    }
    
    // Финальный результат
    out <== outTemp[n];
}


