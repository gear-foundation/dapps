type SlotPositions = {
  player: { top: number; left?: number; right?: number };
  cards: { top: number; left?: number; right?: number };
};

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

const getHorizontalOffset = (
  slotsCount: number,
  index: number,
): Record<'player' | 'cards', { left?: number; right?: number }> => {
  const isEven = slotsCount % 2 === 0;
  const isLeft = index < slotsCount / 2;
  const isCenter = !isEven && index === Math.floor(slotsCount / 2);

  if (isCenter) {
    return { player: { left: 115 }, cards: { left: 208 } };
  }

  if (isLeft) {
    return { player: { left: -23 }, cards: { left: 70 } };
  }

  return { player: { right: -23 }, cards: { right: 67 } };
};

const getSlotPositions = (totalPositions: number): SlotPositions[] => {
  const slotPositions = topOffsetConfig[totalPositions];

  if (!slotPositions) {
    throw new Error('Invalid number of positions');
  }

  return slotPositions.map(([playerTopOffset, cardsTopOffset], index) => {
    const side = getHorizontalOffset(totalPositions, index);

    return {
      player: { top: playerTopOffset, ...side.player },
      cards: { top: cardsTopOffset, ...side.cards },
    };
  });
};

const commonCardsMarginTop: Record<number, number> = {
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

  if (!marginTop) {
    throw new Error('Invalid number of positions');
  }

  return marginTop;
};

export { getSlotPositions, getCommonCardsMarginTop };
