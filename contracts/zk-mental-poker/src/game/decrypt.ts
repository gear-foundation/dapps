import { CipherCard, ECPoint } from "../types.js";
import { ecPointToHexLE } from "../utils/ec.js";
import { cpProofToBytes } from "../utils/proof.js";
import { toCipherCards, findCardByPoint } from "../utils/cards.js";
import { scalarMul, cpProve, cpVerify, projectiveAdd } from "zk-shuffle-proof";
import { decodeAddress } from "@gear-js/api";

export async function partialDecrypt({
  program, players, playerHands, F, a, d, base, keyrings
}: any) {
  for (let i=0;i<players.length;i++) {
    const partialDecs: any[] = [];
    for (let j=0;j<players.length;j++) {
      if (i===j) continue;
      for (let k=0;k<2;k++) {
        const c0 = playerHands[j][k].c0;
        const skC0 = scalarMul(F, a, d, c0, players[i].sk);
        const delta: ECPoint = { X: F.neg(skC0.X), Y: skC0.Y, Z: skC0.Z };
        const proof = cpProve(F, a, d, base, players[i].pk, c0, skC0, players[i].sk);
        if (!cpVerify(F, a, d, base, players[i].pk, c0, skC0, proof)) throw new Error("Invalid CP proof");
        partialDecs.push({ c0: ecPointToHexLE(c0), delta_c0: ecPointToHexLE(delta), proof: cpProofToBytes(proof) });
      }
    }
    const b = await program.poker.submitPartialDecryptions(partialDecs, null).withAccount(keyrings[i]).calculateGas();
    const r = (await b.withGas(200000000000n).signAndSend()).response;
    console.log(`\nDecryption message sent.\n`);
    console.log(`\nProgram replied: \n\t${JSON.stringify(await r())}`);
  }
}

export async function finalDecryptAndShow({
  program, players, keyrings, F, a, d, cardMap
}: any) {
  for (let i=0;i<players.length;i++) {
    const raw = await program.poker.playerCards(decodeAddress(keyrings[i].address));
    const cards: CipherCard[] = toCipherCards(raw);
    for (let k=0;k<2;k++) {
      const c0 = cards[k].c0, c1 = cards[k].c1;
      const skC0 = scalarMul(F, a, d, c0, players[i].sk);
      const delta: ECPoint = { X: F.neg(skC0.X), Y: skC0.Y, Z: skC0.Z };
      const dec = projectiveAdd(F, a, d, c1, delta);
      const match = findCardByPoint(F, cardMap, dec);
      const label = `Player ${i+1} card ${k+1}`;
      console.log(match ? `${label}: 🃏 ${match.rank} of ${match.suit}` : `${label}: ❓ Unknown card`);
    }
  }
}
