import {
  ClubLgIcon,
  ClubSmIcon,
  DiamondLgIcon,
  DiamondSmIcon,
  HeartLgIcon,
  HeartSmIcon,
  JackBlackIcon,
  JackRedIcon,
  KingBlackIcon,
  KingRedIcon,
  QueenBlackIcon,
  QueenRedIcon,
  SpadeLgIcon,
  SpadeSmIcon,
} from '@/assets/images';
import { Suit } from '@/features/zk/api/types';

const suitLgIcon: Record<Suit, SVGComponent> = {
  Clubs: ClubLgIcon,
  Diamonds: DiamondLgIcon,
  Hearts: HeartLgIcon,
  Spades: SpadeLgIcon,
};

const suitSmIcon: Record<Suit, SVGComponent> = {
  Clubs: ClubSmIcon,
  Diamonds: DiamondSmIcon,
  Hearts: HeartSmIcon,
  Spades: SpadeSmIcon,
};

type RankWithIcon = 'J' | 'Q' | 'K';

const rankIcon: Record<'black' | 'red', Record<RankWithIcon, SVGComponent>> = {
  black: {
    K: KingBlackIcon,
    Q: QueenBlackIcon,
    J: JackBlackIcon,
  },
  red: {
    K: KingRedIcon,
    Q: QueenRedIcon,
    J: JackRedIcon,
  },
};

export { suitLgIcon, suitSmIcon, rankIcon };
