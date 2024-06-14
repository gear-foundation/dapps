export type ZkProofData = {
  proofContent: { a: number[]; b: number[]; c: number[] };
  publicContent: {
    results: [[number], [number]];
    publicHash: number[];
  };
};

export type ZkData = {
  single: Partial<{
    'ships-player': number[][];
    'board-player': string[];
    'board-enemy': string[];
    'proof-data': ZkProofData;
  }>;
};
