import { CrossIcon } from '@/assets/images';
import styles from './map.module.scss';

type Props = {
  sizeBlock: number;
  shipStatusArray: string[];
};

export default function Map({ sizeBlock = 64, shipStatusArray }: Props) {
  const numRows = 5;
  const numCols = 5;

  const renderCell = (row: number, col: number) => {
    const cellIndex = row * numCols + col;
    const cellStatus = shipStatusArray[cellIndex];
    const isShip = cellStatus === 'Ship';
    const isHit = cellStatus === 'Boom';
    const isDeadShips = cellStatus === 'DeadShip';
    const isHitShips = cellStatus === 'BoomShip';

    let cellClassName = `${styles.block} ${isShip ? styles.shipBlock : ''} ${isHitShips ? styles.hitShip : ''} ${
      isHit ? styles.missCell : ''
    }`;

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
      <div key={`block-${row}-${col}`} className={cellClassName} style={cellStyle}>
        {isHit && !isHitShips && !isDeadShips && <div className={styles.hitEmpty} />}
        {(isHitShips || isDeadShips) && <CrossIcon />}
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
    const mapRows = [];
    for (let row = 0; row < numRows; row++) {
      mapRows.push(renderRow(row));
    }
    return mapRows;
  };

  return <>{renderMap()}</>;
}
