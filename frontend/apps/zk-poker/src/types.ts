type Suit = 's' | 'h' | 'd' | 'c';

type Rank = 'A' | 'K' | 'Q' | 'J' | 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2';

type Card = `${Rank}${Suit}`;

type HandRank =
  | 'straight-flush'
  | 'four-of-a-kind'
  | 'full-house'
  | 'flush'
  | 'straight'
  | 'three-of-a-kind'
  | 'two-pair'
  | 'one-pair'
  | 'high-card';

type PlayerStatus = 'bet' | 'fold' | 'all-in' | 'winner' | 'waiting' | 'thinking';

export type { Card, HandRank, Rank, Suit, PlayerStatus };
