export type Proof = {
  pi_a: [string, string];
  pi_b: [[string, string], [string, string]];
  pi_c: [string, string];
};

export declare type FullProof = {
  proof: Proof;
  publicSignals: string[];
};
