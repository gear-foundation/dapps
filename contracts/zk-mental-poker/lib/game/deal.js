export function dealHands(deck, numPlayers) {
    const hands = [];
    for (let i = 0; i < numPlayers; i++) {
        const hand = [];
        for (let j = 0; j < 2; j++) {
            const idx = i * 2 + j;
            const c0 = { X: deck[0][idx], Y: deck[1][idx], Z: deck[2][idx] };
            const c1 = { X: deck[3][idx], Y: deck[4][idx], Z: deck[5][idx] };
            hand.push({ c0, c1 });
        }
        hands.push(hand);
    }
    return hands;
}
