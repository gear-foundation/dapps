pragma circom 2.0.3;
include "node_modules/circomlib/circuits/comparators.circom";
include "node_modules/circomlib/circuits/poseidon.circom";


template IsUniqueArray(N) {
    signal input arr[N];

    for (var i = 0; i < N; i++) {
        for (var j = i + 1; j < N; j++) {
            assert(arr[i] != arr[j]);
        }
    }
}

template NotRightEdge() {
    signal input coord;
    signal output out;

    component is_equal_4 = IsEqual();
    is_equal_4.in[0] <== coord;
    is_equal_4.in[1] <== 4;

    component is_equal_9 = IsEqual();
    is_equal_9.in[0] <== coord;
    is_equal_9.in[1] <== 9;

    component is_equal_14 = IsEqual();
    is_equal_14.in[0] <== coord;
    is_equal_14.in[1] <== 14;

    component is_equal_19 = IsEqual();
    is_equal_19.in[0] <== coord;
    is_equal_19.in[1] <== 19;

    component is_equal_24 = IsEqual();
    is_equal_24.in[0] <== coord;
    is_equal_24.in[1] <== 24;

    out <== (1 - is_equal_4.out - is_equal_9.out - is_equal_14.out - is_equal_19.out - is_equal_24.out);

}

template NotLeftEdge() {
    signal input coord;
    signal output out;

    component is_equal_0 = IsEqual();
    is_equal_0.in[0] <== coord;
    is_equal_0.in[1] <== 0;

    component is_equal_5 = IsEqual();
    is_equal_5.in[0] <== coord;
    is_equal_5.in[1] <== 5;

    component is_equal_10 = IsEqual();
    is_equal_10.in[0] <== coord;
    is_equal_10.in[1] <== 10;

    component is_equal_15 = IsEqual();
    is_equal_15.in[0] <== coord;
    is_equal_15.in[1] <== 15;

    component is_equal_20 = IsEqual();
    is_equal_20.in[0] <== coord;
    is_equal_20.in[1] <== 20;

    out <== (1 - is_equal_0.out - is_equal_5.out - is_equal_10.out - is_equal_15.out - is_equal_20.out);

}

template NotHalfEdge() {
    signal input coord;
    signal output out;

    component is_equal_3 = IsEqual();
    is_equal_3.in[0] <== coord;
    is_equal_3.in[1] <== 3;

    component is_equal_8 = IsEqual();
    is_equal_8.in[0] <== coord;
    is_equal_8.in[1] <== 8;

    component is_equal_13 = IsEqual();
    is_equal_13.in[0] <== coord;
    is_equal_13.in[1] <== 13;

    component is_equal_18 = IsEqual();
    is_equal_18.in[0] <== coord;
    is_equal_18.in[1] <== 18;

    component is_equal_23 = IsEqual();
    is_equal_23.in[0] <== coord;
    is_equal_23.in[1] <== 23;

    out <== (1 - is_equal_3.out - is_equal_8.out - is_equal_13.out - is_equal_18.out - is_equal_23.out);

}

template CheckDistance(N) {
    signal input coord;
    signal input arr[N];
    signal cells[9];

    component not_left_edge = NotLeftEdge();
    not_left_edge.coord <== coord;

    component not_right_edge = NotRightEdge();
    not_right_edge.coord <== coord;

    cells[0] <== coord;
    cells[1] <== coord-6*not_left_edge.out;
    cells[2] <== coord-5;
    cells[3] <== coord-4*not_right_edge.out;
    cells[4] <== coord-1*not_left_edge.out;
    cells[5] <== coord+1*not_right_edge.out;
    cells[6] <== coord+4*not_left_edge.out;
    cells[7] <== coord+5;
    cells[8] <== coord+6*not_right_edge.out;

    component not_edge = NotRightEdge();
    not_edge.coord <== arr[0];

    for (var i = 0; i < 8; i++) {
        for (var j = 0; j < N; j++) {
            assert(cells[i] != arr[j]);
        }
    }
    
}

template IntegrityEdgeSize2() {
    signal input arr[2];

    component not_edge = NotRightEdge();
    not_edge.coord <== arr[0];

    var horizontal = (arr[1] == arr[0] + 1) * not_edge.out;
    var vertical = (arr[1] == arr[0] + 5);
    assert((horizontal+vertical) == 1);
    
}

template IntegrityEdgeSize3() {
    signal input arr[3];

    component not_edge = NotRightEdge();
    not_edge.coord <== arr[0];

    component not_half_edge = NotHalfEdge();
    not_half_edge.coord <== arr[0];

    var horizontal = (arr[1] == arr[0] + 1) * not_edge.out * not_half_edge.out;
    var vertical = (arr[1] == arr[0] + 5);
    assert((horizontal+vertical) == 1);
    
}

template Integrity(N) {
    signal input arr[N];

    for (var i = 1; i < N; i++) {        
        var horizontal = (arr[i] == arr[i - 1] + 1);
        var vertical = (arr[i] == arr[i - 1] + 5);
        assert((horizontal+vertical) == 1);
    }
}

template BattleshipPlacement(N1, N2, N3, N4) {
    signal input ship_1[N1];
    signal input ship_2[N2];
    signal input ship_3[N3];
    signal input ship_4[N4];
    signal input hash;

    signal combinedShips[N1 + N2 + N3 + N4];

    // combine all the ship arrays into one
    for (var i = 0; i < N1; i++) {
        combinedShips[i] <== ship_1[i];
    }
    for (var i = 0; i < N2; i++) {
        combinedShips[N1 + i] <== ship_2[i];
    }
    for (var i = 0; i < N3; i++) {
        combinedShips[N1 + N2 + i] <== ship_3[i];
    }
    for (var i = 0; i < N4; i++) {
        combinedShips[N1 + N2 + N3 + i] <== ship_4[i];
    }

    // check hash
    component poseidon = Poseidon(N1 + N2 + N3 + N4);
    for (var i = 0; i < N1 + N2 + N3 + N4; i++) {
        poseidon.inputs[i] <== combinedShips[i];
    }
    // log(poseidon.out);
    hash === poseidon.out;

    // check the range of values (no more than 24)
    for (var i = 0; i < N1 + N2 + N3 + N4; i++) {
        assert(combinedShips[i] < 25); // check outfield 
    }

    // checking that the ship doesn't break up and go to the other side
    component integrity_2 = IntegrityEdgeSize2();
    for (var i = 0; i < N2; i++) {
        integrity_2.arr[i] <== ship_2[i];
    }
    component integrity_3 = IntegrityEdgeSize2();
    for (var i = 0; i < N3; i++) {
        integrity_3.arr[i] <== ship_3[i];
    }
    component integrity_4 = Integrity(N4);
    for (var i = 0; i < N4; i++) {
        integrity_4.arr[i] <== ship_4[i];
    }
    component integrity_edge_4 = IntegrityEdgeSize3();
    for (var i = 0; i < N4; i++) {
        integrity_edge_4.arr[i] <== ship_4[i];
    }

    // check the distance between ships
    // 1. the first ship relative to ship_2, ship_3 and ship_4
    signal combinedShips1[N2 + N3 + N4];
    for (var i = 0; i < N2; i++) {
        combinedShips1[i] <== ship_2[i];
    }
    for (var i = 0; i < N3; i++) {
        combinedShips1[N2 + i] <== ship_3[i];
    }
    for (var i = 0; i < N4; i++) {
        combinedShips1[N2 + N3 + i] <== ship_4[i];
    }

    component check_distance_1 = CheckDistance(N2 + N3 + N4);
    check_distance_1.coord <== ship_1[0];
    for (var i = 0; i < N2 + N3 + N4; i++) {
        check_distance_1.arr[i] <== combinedShips1[i];
    }

    // 2. the second ship relative to ship_3 and ship_4
    signal combinedShips2[N3 + N4];
    for (var i = 0; i < N3; i++) {
        combinedShips2[i] <== ship_3[i];
    }
    for (var i = 0; i < N4; i++) {
        combinedShips2[N3 + i] <== ship_4[i];
    }

    component check_distance_2 = CheckDistance(N3 + N4);
    check_distance_2.coord <== ship_2[0];
    for (var i = 0; i < N3 + N4; i++) {
        check_distance_2.arr[i] <== combinedShips2[i];
    }

    component check_distance_3 = CheckDistance(N3 + N4);
    check_distance_3.coord <== ship_2[1];
    for (var i = 0; i < N3 + N4; i++) {
        check_distance_3.arr[i] <== combinedShips2[i];
    }

    // 3. the third ship relative to ship_4
    component check_distance_4 = CheckDistance(N4);
    check_distance_4.coord <== ship_3[0];
    for (var i = 0; i < N4; i++) {
        check_distance_4.arr[i] <== ship_4[i];
    }

    component check_distance_5 = CheckDistance(N4);
    check_distance_5.coord <== ship_2[1];
    for (var i = 0; i < N4; i++) {
        check_distance_5.arr[i] <== ship_4[i];
    }
}

component main {public [hash]} = BattleshipPlacement(1, 2, 2, 3);
