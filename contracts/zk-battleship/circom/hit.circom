pragma circom 2.0.3;
include "node_modules/circomlib/circuits/comparators.circom";
include "node_modules/circomlib/circuits/poseidon.circom";
include "node_modules/circomlib/circuits/mux1.circom";
include "node_modules/circomlib/circuits/mux2.circom";

template BattleshipHit(N1, N2, N3, N4, MAX_HITS) {
    signal input ship_1[N1];
    signal input ship_2[N2];
    signal input ship_3[N3];
    signal input ship_4[N4];
    signal input hits[MAX_HITS];  // Previous hits
    signal input hit;             // Current shot coordinate
    signal input hash;            // Hash on each element of coordinates
    signal output is_hit;         // 0 = miss, 1 = hit, 2 = sunk

    assert(hit < 25);            // Check outfield

    // combine all the ship arrays into one
    signal combinedShips[N1 + N2 + N3 + N4];
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

    for (var i = 0; i < MAX_HITS; i++) {
        assert(hits[i] != hit);
    }

    // Check hits on ship 1
    var hits_on_ship1 = 0;
    component is_equal_ship1[N1];
    for (var j = 0; j < N1; j++) {
        assert(ship_1[j] < 25);  // Check outfield
        is_equal_ship1[j] = IsEqual();
        is_equal_ship1[j].in[0] <== hit;
        is_equal_ship1[j].in[1] <== ship_1[j];
        hits_on_ship1 += is_equal_ship1[j].out;
    }
    signal hit_ship1 <== hits_on_ship1;

    // Check previous hits on ship 1
    var previous_hits_on_ship1 = 0;
    component is_equal_previous1[MAX_HITS][N1];
    for (var i = 0; i < MAX_HITS; i++) {
        for (var j = 0; j < N1; j++) {
            is_equal_previous1[i][j] = IsEqual();
            is_equal_previous1[i][j].in[0] <== hits[i];
            is_equal_previous1[i][j].in[1] <== ship_1[j];
            previous_hits_on_ship1 += is_equal_previous1[i][j].out;
        }
    }

    // Total hits on ship 1
    signal total_hits_on_ship1;
    total_hits_on_ship1 <== previous_hits_on_ship1 + hits_on_ship1;

    component ge_total_hits1 = GreaterEqThan(N1);
    ge_total_hits1.in[0] <== total_hits_on_ship1;
    ge_total_hits1.in[1] <== N1;
    signal sunk_ship1 <== ge_total_hits1.out;

    // Check hits on ship 2
    var hits_on_ship2 = 0;
    component is_equal_ship2[N2];
    for (var j = 0; j < N2; j++) {
        assert(ship_2[j] < 25);  // Check outfield
        is_equal_ship2[j] = IsEqual();
        is_equal_ship2[j].in[0] <== hit;
        is_equal_ship2[j].in[1] <== ship_2[j];
        hits_on_ship2 += is_equal_ship2[j].out;
    }
    signal hit_ship2 <== hits_on_ship2;

    // Check previous hits on ship 2
    var previous_hits_on_ship2 = 0;
    component is_equal_previous2[MAX_HITS][N2];
    for (var i = 0; i < MAX_HITS; i++) {
        for (var j = 0; j < N2; j++) {
            is_equal_previous2[i][j] = IsEqual();
            is_equal_previous2[i][j].in[0] <== hits[i];
            is_equal_previous2[i][j].in[1] <== ship_2[j];
            previous_hits_on_ship2 += is_equal_previous2[i][j].out;
        }
    }

    // Total hits on ship 2
    signal total_hits_on_ship2;
    total_hits_on_ship2 <== previous_hits_on_ship2 + hits_on_ship2;

    component ge_total_hits2 = GreaterEqThan(N2);
    ge_total_hits2.in[0] <== total_hits_on_ship2;
    ge_total_hits2.in[1] <== N2;
    signal sunk_ship2 <== ge_total_hits2.out;

    // Check hits on ship 3
    var hits_on_ship3 = 0;
    component is_equal_ship3[N3];
    for (var j = 0; j < N3; j++) {
        assert(ship_3[j] < 25);  // Check outfield
        is_equal_ship3[j] = IsEqual();
        is_equal_ship3[j].in[0] <== hit;
        is_equal_ship3[j].in[1] <== ship_3[j];
        hits_on_ship3 += is_equal_ship3[j].out;
    }
    signal hit_ship3 <== hits_on_ship3;

    // Check previous hits on ship 3
    var previous_hits_on_ship3 = 0;
    component is_equal_previous3[MAX_HITS][N3];
    for (var i = 0; i < MAX_HITS; i++) {
        for (var j = 0; j < N3; j++) {
            is_equal_previous3[i][j] = IsEqual();
            is_equal_previous3[i][j].in[0] <== hits[i];
            is_equal_previous3[i][j].in[1] <== ship_3[j];
            previous_hits_on_ship3 += is_equal_previous3[i][j].out;
        }
    }

    // Total hits on ship 3
    signal total_hits_on_ship3;
    total_hits_on_ship3 <== previous_hits_on_ship3 + hits_on_ship3;

    component ge_total_hits3 = GreaterEqThan(N3);
    ge_total_hits3.in[0] <== total_hits_on_ship3;
    ge_total_hits3.in[1] <== N3;
    signal sunk_ship3 <== ge_total_hits3.out;

    // Check hits on ship 4
    var hits_on_ship4 = 0;
    component is_equal_ship4[N4];
    for (var j = 0; j < N4; j++) {
        assert(ship_4[j] < 25);  // Check outfield
        is_equal_ship4[j] = IsEqual();
        is_equal_ship4[j].in[0] <== hit;
        is_equal_ship4[j].in[1] <== ship_4[j];
        hits_on_ship4 += is_equal_ship4[j].out;
    }
    signal hit_ship4 <== hits_on_ship4;

    // Check previous hits on ship 4
    var previous_hits_on_ship4 = 0;
    component is_equal_previous4[MAX_HITS][N4];
    for (var i = 0; i < MAX_HITS; i++) {
        for (var j = 0; j < N4; j++) {
            is_equal_previous4[i][j] = IsEqual();
            is_equal_previous4[i][j].in[0] <== hits[i];
            is_equal_previous4[i][j].in[1] <== ship_4[j];
            previous_hits_on_ship4 += is_equal_previous4[i][j].out;
        }
    }

    // Total hits on ship 4
    signal total_hits_on_ship4;
    total_hits_on_ship4 <== previous_hits_on_ship4 + hits_on_ship4;

    component ge_total_hits4 = GreaterEqThan(N4);
    ge_total_hits4.in[0] <== total_hits_on_ship4;
    ge_total_hits4.in[1] <== N4;
    signal sunk_ship4 <== ge_total_hits4.out;

    // Define is_hit using Mux1 components
    component mux_hit1 = Mux1();
    mux_hit1.c[0] <== 1;
    mux_hit1.c[1] <== 2;
    mux_hit1.s <== sunk_ship1;

    component mux_hit2 = Mux1();
    mux_hit2.c[0] <== 1;
    mux_hit2.c[1] <== 2;
    mux_hit2.s <== sunk_ship2;

    component mux_hit3 = Mux1();
    mux_hit3.c[0] <== 1;
    mux_hit3.c[1] <== 2;
    mux_hit3.s <== sunk_ship3;

    component mux_hit4 = Mux1();
    mux_hit4.c[0] <== 1;
    mux_hit4.c[1] <== 2;
    mux_hit4.s <== sunk_ship4;

    component mux_main1 = Mux1();
    mux_main1.c[0] <== 0;
    mux_main1.c[1] <== mux_hit1.out;
    mux_main1.s <== hit_ship1;

    component mux_main2 = Mux1();
    mux_main2.c[0] <== mux_main1.out;
    mux_main2.c[1] <== mux_hit2.out;
    mux_main2.s <== hit_ship2;

    component mux_main3 = Mux1();
    mux_main3.c[0] <== mux_main2.out;
    mux_main3.c[1] <== mux_hit3.out;
    mux_main3.s <== hit_ship3;

    component mux_main4 = Mux1();
    mux_main4.c[0] <== mux_main3.out;
    mux_main4.c[1] <== mux_hit4.out;
    mux_main4.s <== hit_ship4;

    log(mux_main4.out);

    is_hit <== mux_main4.out;

}

// Define the main component
component main {public [hit, hash]} = BattleshipHit(1, 2, 2, 3, 25);

