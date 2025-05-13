// ! TODO: add proof bytes
type ProofBytes = unknown;

export type ZkProofData = {
  proofContent: ProofBytes;
  publicContent: {
    results: [[number], [number]];
    publicHash: `0x${string}`;
  };
};
