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
import { Suit } from '@/types';

const suitLgIcon: Record<Suit, SVGComponent> = {
  c: ClubLgIcon,
  d: DiamondLgIcon,
  h: HeartLgIcon,
  s: SpadeLgIcon,
};

const suitSmIcon: Record<Suit, SVGComponent> = {
  c: ClubSmIcon,
  d: DiamondSmIcon,
  h: HeartSmIcon,
  s: SpadeSmIcon,
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
