type Side = 'left' | 'right' | 'top';

// [playerTopOffset, cardsTopOffset]
const topOffsetConfig: Record<number, [number, number][]> = {
  0: [],
  1: [[-17, 36]],
  2: [
    [64, 149],
    [64, 149],
  ],
  3: [
    [336, 389],
    [-17, 36],
    [336, 389],
  ],
  4: [
    [309, 394],
    [64, 149],
    [64, 149],
    [309, 394],
  ],
  5: [
    [336, 389],
    [146, 158],
    [-17, 36],
    [146, 158],
    [336, 389],
  ],
  6: [
    [392, 445],
    [229, 314],
    [48, 101],
    [48, 101],
    [229, 314],
    [392, 445],
  ],
  7: [
    [392, 445],
    [261, 346],
    [130, 142],
    [-17, 36],
    [130, 142],
    [261, 346],
    [392, 445],
  ],
  8: [
    [392, 445],
    [261, 346],
    [130, 142],
    [1, 52],
    [1, 52],
    [130, 142],
    [261, 346],
    [392, 445],
  ],
};

const getPositionSide = (slotsCount: number, index: number): Side => {
  const isEven = slotsCount % 2 === 0;

  if (!isEven && index === Math.floor(slotsCount / 2)) {
    return 'top';
  }

  if (index < slotsCount / 2) {
    return 'left';
  }

  return 'right';
};

const getSlotPositions = (totalPositions: number): [number, number][] => {
  const slotPositions = topOffsetConfig[totalPositions];

  if (!slotPositions) {
    throw new Error('Invalid number of positions');
  }

  return slotPositions;
};

const commonCardsMarginTop: Record<number, number> = {
  0: 143.3,
  1: 143.3,
  2: 176.3,
  3: 143.3,
  4: 176.3,
  5: 175.3,
  6: 112.3,
  7: 151.3,
  8: 151.3,
};

const getCommonCardsMarginTop = (totalPositions: number): number => {
  const marginTop = commonCardsMarginTop[totalPositions];

  if (marginTop === undefined) {
    throw new Error('Invalid number of positions');
  }

  return marginTop;
};

export { getSlotPositions, getCommonCardsMarginTop, getPositionSide };
