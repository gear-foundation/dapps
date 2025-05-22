/* eslint-disable */

//import { Proof, packToSolidityProof, SolidityProof } from "@semaphore-protocol/proof";
import { groth16 } from 'snarkjs';

//export { packToSolidityProof, SolidityProof };

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
  // @ts-ignore
  return <FullProof>await groth16.fullProve({ Y, skP, pkP }, wasmFile, zkeyFile);
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
  // @ts-ignore
  return <FullProof>await groth16.fullProve(
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
