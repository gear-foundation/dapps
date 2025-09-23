//import { Proof, packToSolidityProof, SolidityProof } from "@semaphore-protocol/proof";
const snarkjs = require("snarkjs");

//export { packToSolidityProof, SolidityProof };

type Contract = any;

export type Proof = {
    pi_a: [string, string];
    pi_b: [[string, string], [string, string]];
    pi_c: [string, string];
  };

export declare type FullProof = {
  proof: Proof;
  publicSignals: string[];
};

// Generates proof for decryption circuit.
export async function generateDecryptProof(
  Y: bigint[],
  skP: bigint,
  pkP: bigint[],
  wasmFile: string,
  zkeyFile: string,
): Promise<FullProof> {
  // eslint-disable-next-line keyword-spacing
  return <FullProof>await snarkjs.groth16.fullProve({ Y, skP, pkP }, wasmFile, zkeyFile);
}

// Generates proof for shuffle encrypt v2 circuit.
export async function generateShuffleEncryptV2Proof(
  pk: bigint[],
  A: bigint[],
  R: bigint[],
  UX0: bigint[],
  UX1: bigint[],
  UDelta0: bigint[],
  UDelta1: bigint[],
  s_u: bigint[],
  VX0: bigint[],
  VX1: bigint[],
  VDelta0: bigint[],
  VDelta1: bigint[],
  s_v: bigint[],
  wasmFile: string,
  zkeyFile: string,
): Promise<FullProof> {
  // eslint-disable-next-line keyword-spacing
  return <FullProof>await snarkjs.groth16.fullProve(
    {
      pk,
      A,
      R,
      UX0,
      UX1,
      UDelta0,
      UDelta1,
      VX0,
      VX1,
      VDelta0,
      VDelta1,
      s_u,
      s_v,
    },
    wasmFile,
    zkeyFile,
  );
}

// // Queries the current deck from contract, shuffles & generates ZK proof locally, and updates the deck on contract.
// export async function shuffle(
//   babyjub: BabyJub,
//   A: bigint[],
//   R: bigint[],
//   aggregatedPk: bigint[],
//   numCards: number,
//   gameId: number,
//   playerAddr: string,
//   gameContract: SignerWithAddress,
//   stateMachineContract: Contract,
//   shuffleEncryptV2WasmFile: string,
//   shuffleEncryptV2ZkeyFile: string,
// ) {
//   const deck: Deck = await stateMachineContract.queryDeck(gameId);
//   const aggregatedPkEC = [babyjub.F.e(aggregatedPk[0]), babyjub.F.e(aggregatedPk[1])];
//   const preprocessedDeck = prepareShuffleDeck(babyjub, deck, numCards);
//   const plaintext_output = shuffleEncryptV2Plaintext(
//     babyjub,
//     numCards,
//     A,
//     R,
//     aggregatedPkEC,
//     preprocessedDeck.X0,
//     preprocessedDeck.X1,
//     preprocessedDeck.Delta[0],
//     preprocessedDeck.Delta[1],
//     preprocessedDeck.Selector,
//   );
//   const shuffleEncryptV2Output = await generateShuffleEncryptV2Proof(
//     aggregatedPk,
//     A,
//     R,
//     preprocessedDeck.X0,
//     preprocessedDeck.X1,
//     preprocessedDeck.Delta[0],
//     preprocessedDeck.Delta[1],
//     preprocessedDeck.Selector,
//     plaintext_output.X0,
//     plaintext_output.X1,
//     plaintext_output.delta0,
//     plaintext_output.delta1,
//     plaintext_output.selector,
//     shuffleEncryptV2WasmFile,
//     shuffleEncryptV2ZkeyFile,
//   );
//   const solidityProof: SolidityProof = packToSolidityProof(shuffleEncryptV2Output.proof);
//   await stateMachineContract
//     .connect(gameContract)
//     .shuffle(
//       playerAddr,
//       solidityProof,
//       shuffleEncryptV2Output.publicSignals.slice(3 + numCards * 2, 3 + numCards * 3),
//       shuffleEncryptV2Output.publicSignals.slice(3 + numCards * 3, 3 + numCards * 4),
//       [
//         shuffleEncryptV2Output.publicSignals[5 + numCards * 4],
//         shuffleEncryptV2Output.publicSignals[6 + numCards * 4],
//       ],
//       gameId,
//     );
// }

