import { ProofBytes } from '@/app/utils/sails/lib/lib';

export type ZkProofData = {
  proofContent: ProofBytes;
  publicContent: {
    results: [[number], [number]];
    publicHash: `0x${string}`;
  };
};

export type ZkData = Partial<{
  single: Partial<{
    'ships-player': number[][];
    'hits-player': number[];
    'board-player': string[];
    'board-enemy': string[];
    'proof-data': ZkProofData;
  }>;
  multi: Partial<{
    'ships-player': number[][];
    'hits-player': number[];
    'board-player': string[];
    'board-enemy': string[];
    'proof-data': ZkProofData;
  }>;
}>;

export type GameType = 'single' | 'multi';
