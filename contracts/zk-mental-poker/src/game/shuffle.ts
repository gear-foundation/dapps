import { EncryptedCard } from "../types.js";
import { numberToLittleEndianBytes } from "../utils/bytes.js";
import { buildCardMap } from "../utils/cards.js";
import { encodeProof, publicSignalsToBytes } from "../utils/proof.js";
import { elgamalEncryptDeck, generatePermutation, permuteMatrix } from "zk-shuffle-proof";

export async function shuffleDeckWithProofs({
  players, numCards, F, a, d, base, aggKey, deck,
  encryptWasmFile, encryptZkeyFile, encryptVkey,
  groth16, snarkjs, program, adminKeyring
}: any) {
  const cardMap = buildCardMap(deck);
  const proofs: any[] = [];

  for (let i = 0; i < players.length; i++) {
    console.log(`\nPlayer ${i + 1} shuffling...`);
    const permutation = generatePermutation(numCards);
    const { encrypted, rScalars } = elgamalEncryptDeck(F, a, d, base, aggKey, deck);
    const shuffled = permuteMatrix(encrypted, permutation);

    const input = {
      pk: [aggKey.X.toString(), aggKey.Y.toString(), aggKey.Z.toString()],
      R: rScalars.map((r: bigint) => r.toString()),
      original: deck.map((row: bigint[]) => row.map((v) => v.toString())),
      permuted: shuffled.map((row: bigint[]) => row.map((v) => v.toString())),
    };

    console.time("fullProve");
    const { proof, publicSignals } = await groth16.fullProve(input, encryptWasmFile, encryptZkeyFile);
    console.timeEnd("fullProve");

    const isValid = await snarkjs.groth16.verify(encryptVkey, publicSignals, proof);
    if (!isValid) throw new Error(`Invalid shuffle proof from Player ${i + 1}`);

    proofs.push({ proof_bytes: encodeProof(proof), public_input: publicSignalsToBytes(publicSignals) });
    deck = shuffled;
  }

  const encrypted_deck: Array<EncryptedCard> = [];
  for (let i=0;i<numCards;i++) {
    encrypted_deck.push({
      c0: [ numberToLittleEndianBytes(deck[0][i]), numberToLittleEndianBytes(deck[1][i]), numberToLittleEndianBytes(deck[2][i]) ],
      c1: [ numberToLittleEndianBytes(deck[3][i]), numberToLittleEndianBytes(deck[4][i]), numberToLittleEndianBytes(deck[5][i]) ],
    });
  }

  const shuffleBuilder = await program.poker.shuffleDeck(encrypted_deck, proofs).withAccount(adminKeyring).calculateGas();
  const shuffleResponse = (await shuffleBuilder.withGas(500000000000n).signAndSend()).response;
  const shuffleReply = await shuffleResponse();
  console.log(`\nProgram replied: \n\t${JSON.stringify(shuffleReply)}`);

  return { deck, cardMap };
}
