import clsx from 'clsx';
import { useEffect, useState } from 'react';

import fireGif from '@/assets/images/fire.gif';
import CellCrossSVG from '@/assets/images/icons/cross.svg?react';
import shipX1SVG from '@/assets/images/icons/ship-x1.svg';
import shipX2SVG from '@/assets/images/icons/ship-x2.svg';
import shipX3SVG from '@/assets/images/icons/ship-x3.svg';
import shipX4SVG from '@/assets/images/icons/ship-x4.svg';
import seaPng from '@/assets/images/sea.png';
import smokeSVG from '@/assets/images/smoke.gif';
import { Text } from '@/components/ui/text';

import { RenderShips, RenderedShip } from '../../types';
import { Timer } from '../timer';

import styles from './map.module.scss';

type Props = {
  sizeBlock: number;
  shipStatusArray: string[];
  onClickCell: (_: number) => void;
  isDisabledCell: boolean;
  onDefineDeadShip: (deadShips: RenderShips) => void;
  lastHit: number | null;
  showTimer?: boolean;
  remainingTime: string | number | bigint | null | undefined;
};

type MarkedShips = {
  [key: string]: 1 | 0;
};

export default function MapEnemy({
  sizeBlock = 64,
  shipStatusArray,
  onClickCell,
  isDisabledCell,
  onDefineDeadShip,
  lastHit,
  showTimer,
  remainingTime,
}: Props) {
  const numRows = 5;
  const numCols = 5;
  const [deadShips, setDeadShips] = useState<RenderShips>({});
  const ships: { [key: string]: string } = {
    shipX1SVG,
    shipX2SVG,
    shipX3SVG,
    shipX4SVG,
  };

  const handleCellClick = (cellIndex: number) => {
    if (!isDisabledCell && !['Boom', 'BoomShip', 'DeadShip'].includes(shipStatusArray[cellIndex])) {
      onClickCell(cellIndex);
    }
  };

  const definedDeadShips = (board: string[]) => {
    const markedShips: MarkedShips = {};

    const defineShip = (index: number, step: number): [number, number] => {
      markedShips[index] = 1;

      if (board[index + 1] === 'DeadShip' && !markedShips[index + 1] && (index + 1) % numCols !== 0) {
        const [length] = defineShip(index + 1, step + 1);
        return [length, 0];
      }

      if (board[index + numCols] === 'DeadShip' && !markedShips[index + numCols]) {
        const [length] = defineShip(index + numCols, step + 1);
        return [length, 90];
      }

      return [step, 0];
    };

    for (let i = 0; i < board.length; i++) {
      if (board[i] === 'DeadShip' && !markedShips[i]) {
        const [length, degrees] = defineShip(i, 1);

        setDeadShips((prev) => ({
          ...prev,
          [i]: {
            length,
            degrees,
          },
        }));
      }
    }
  };

  const handleRenderDeadShip = (ship: RenderedShip) => {
    const translateShip = (length: number) => {
      if (length === 1) return 0;
      if (length === 2) return 25;
      if (length === 3) return 33;

      return 50;
    };

    const { length, degrees } = ship;

    return (
      <img
        alt="Destroyed ship"
        src={ships[`shipX${length}SVG`]}
        style={{
          position: 'absolute',
          top: 0,
          height: sizeBlock,
          transform: `rotate(${degrees}deg) translateX(${translateShip(length)}%)`,
        }}
      />
    );
  };

  useEffect(() => {
    definedDeadShips(shipStatusArray);
  }, [shipStatusArray]);

  useEffect(() => {
    if (Object.keys(deadShips).length) {
      onDefineDeadShip(deadShips);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [deadShips]);

  const renderCell = (row: number, col: number) => {
    const cellIndex = row * numCols + col;
    const cellStatus = shipStatusArray[cellIndex];

    const isHit = cellStatus === 'Boom';
    const isDeadShips = cellStatus === 'DeadShip';
    const isHitShips = cellStatus === 'BoomShip';
    const isPending = lastHit === cellIndex && cellStatus === 'Unknown';
    const isUnavailable = isDisabledCell || ['Boom', 'BoomShip', 'DeadShip'].includes(cellStatus);

    let cellClassName = `${styles.block}`;

    if (isHit) {
      cellClassName += ` ${styles.hitCell}`;
    }

    if (isDeadShips) {
      cellClassName += ` ${styles.deadShip} ${styles.deadShipEnemy}`;
    }

    if (isHitShips) {
      cellClassName += ` ${styles.hitShip} ${styles.hitShipEnemy}`;
    }

    const cellStyle = {
      width: `${sizeBlock}px`,
      height: `${sizeBlock}px`,
    };

    const statusDescription = (() => {
      switch (cellStatus) {
        case 'Unknown':
          return 'unknown';
        case 'Boom':
          return 'missed shot';
        case 'BoomShip':
          return 'successful hit';
        case 'DeadShip':
          return 'destroyed ship';
        default:
          return 'untouched';
      }
    })();

    return (
      <button
        type="button"
        key={`block-${row}-${col}`}
        className={clsx(cellClassName, styles.blockEnemy, isDisabledCell && styles.blockDisabled)}
        style={cellStyle}
        onClick={() => handleCellClick(cellIndex)}
        disabled={isUnavailable}
        aria-label={`Cell ${row + 1}-${col + 1}: ${statusDescription}`}>
        {isHit && !isDeadShips && !isHitShips && <div className={styles.hitCircle} />}
        {isDeadShips && !!deadShips[cellIndex] && handleRenderDeadShip(deadShips[cellIndex])}
        {(isDeadShips || isHitShips) && (
          <>
            <CellCrossSVG className={clsx(styles.cellCross, styles.cellCrossEnemy)} />
            <img src={fireGif} alt="fire" className={styles.cellFire} />
            {Math.random() >= 0.5 && <img src={smokeSVG} alt="smoke" className={styles.cellSmoke} />}
          </>
        )}
        {isPending && <div className={styles.pendingCell} />}
      </button>
    );
  };

  const renderRow = (row: number) => {
    const rowBlocks = [];
    for (let col = 0; col < numCols; col++) {
      rowBlocks.push(renderCell(row, col));
    }
    return (
      <div key={`row-${row}`} className={styles.row}>
        {rowBlocks}
      </div>
    );
  };

  const renderMap = () => {
    const map = [];
    for (let row = 0; row < numRows; row++) {
      map.push(renderRow(row));
    }
    return map;
  };

  return (
    <div className={styles.container}>
      <img src={seaPng} alt="sea" className={styles.sea} />
      {renderMap()}
      {showTimer && (
        <div className={styles.oponentsTurn}>
          <Text size="sm">Opponent&apos;s Turn:</Text>
          <Text weight="semibold" className={styles.time}>
            <Timer remainingTime={remainingTime} shouldGoOn />
          </Text>
        </div>
      )}
    </div>
  );
}
