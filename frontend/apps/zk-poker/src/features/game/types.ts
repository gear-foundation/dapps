export type HandRank =
  | 'straight flush'
  | 'four of a kind'
  | 'full house'
  | 'flush'
  | 'straight'
  | 'three of a kind'
  | 'two pair'
  | 'one pair'
  | 'high card';

export type PlayerStatus = 'bet' | 'fold' | 'all-in' | 'winner' | 'check' | 'waiting' | 'thinking';
