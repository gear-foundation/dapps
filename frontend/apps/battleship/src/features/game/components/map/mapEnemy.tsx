import { CrossIcon } from '@/assets/images';
import styles from './map.module.scss';

type Props = {
  sizeBlock: number;
  shipStatusArray: string[];
  onClickCell: (_: number) => void;
  isDisabledCell: boolean;
};

export default function MapEnemy({
  sizeBlock = 64,
  shipStatusArray,
  onClickCell,
  isDisabledCell,
}: Props) {
  const numRows = 5;
  const numCols = 5;

  const handleCellClick = (cellIndex: number) => {
    if (!isDisabledCell) {
      onClickCell(cellIndex);
    }
  };

  const renderCell = (row: number, col: number) => {
    const cellIndex = row * numCols + col;
    const cellStatus = shipStatusArray[cellIndex];

    const isHit = cellStatus === 'Boom';
    const isDeadShips = cellStatus === 'DeadShip';
    const isHitShips = cellStatus === 'BoomShip';

    let cellClassName = `${styles.block}`;

    if (isDeadShips) {
      cellClassName += ` ${styles.deadShip}`;
    }

    if (isHitShips) {
      cellClassName += ` ${styles.hitShip}`;
    }

    const cellStyle = {
      width: `${sizeBlock}px`,
      height: `${sizeBlock}px`,
    };

    return (
      <div
        key={`block-${row}-${col}`}
        className={cellClassName}
        style={cellStyle}
        onClick={() => handleCellClick(cellIndex)}>
        {(isHit || isHitShips || isDeadShips) && <CrossIcon />}
      </div>
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

  return <>{renderMap()}</>;
}
