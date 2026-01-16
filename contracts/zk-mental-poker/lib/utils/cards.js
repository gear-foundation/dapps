import { SUITS, RANKS } from "../config.js";
import { toAffine } from "./ec.js";
import { littleEndianHexToBigInt } from "./bytes.js";
export function buildCardMap(deck) {
    const n = SUITS.length * RANKS.length;
    if (deck[0].length !== n)
        throw new Error(`Deck size mismatch: expected ${n}, got ${deck[0].length}`);
    const cards = [];
    for (let s = 0; s < SUITS.length; s++)
        for (let r = 0; r < RANKS.length; r++) {
            const i = s * RANKS.length + r;
            cards.push({ suit: SUITS[s], rank: RANKS[r], point: { X: deck[3][i], Y: deck[4][i], Z: deck[5][i] } });
        }
    return cards;
}
export function findCardByPoint(F, cards, target) {
    const t = toAffine(F, target);
    return cards.find(c => {
        const cc = toAffine(F, c.point);
        return F.eq(cc.x, t.x) && F.eq(cc.y, t.y);
    });
}
export function toCipherCards(data) {
    return data.map(({ c0, c1 }) => ({
        c0: { X: littleEndianHexToBigInt(c0[0]), Y: littleEndianHexToBigInt(c0[1]), Z: littleEndianHexToBigInt(c0[2]) },
        c1: { X: littleEndianHexToBigInt(c1[0]), Y: littleEndianHexToBigInt(c1[1]), Z: littleEndianHexToBigInt(c1[2]) },
    }));
}
