import { Icon } from '../../ui/icon';
import clsx from 'clsx';
import { getBgColors, isPartialSubset } from 'app/utils';
import { DominoItem } from '../../common/domino-item';
import { DominoZone } from '../../common/domino-zone';
import { DominoTileType } from 'app/types/game';
import { useEffect, useRef, useState } from 'react';
import { useApp, useGame } from '../../../app/context';
import { PlayerTrain } from '../../common/player-train';
import { useAccount } from '@gear-js/react-hooks';
import { useRefDimensions } from '../../../app/hooks/use-ref-dimensions';
import { TooltipWrapper } from '@gear-js/ui';

type Props = {
  index: number;
  train?: boolean;
  isUserTrain?: boolean;
  active?: boolean;
  tiles?: DominoTileType[];
};

const SPACING = 2;
const CARD_WIDTH = 72;

export const PlayerTrackSection = ({ index, train, isUserTrain, active, tiles }: Props) => {
  const { account } = useAccount();
  const { isAllowed } = useApp();
  const { gameWasm: wasm, playerChoice } = useGame();
  const [isDisabled, setIsDisabled] = useState(false);
  const ref = useRef<HTMLDivElement>(null);
  const [w] = useRefDimensions(ref);
  const [arr, setArr] = useState<DominoTileType[] | undefined>(tiles);

  useEffect(() => {
    if (tiles) {
      const temp = tiles;
      const space = Math.floor(w / (CARD_WIDTH + (tiles?.length - 1) * SPACING));

      if (tiles.length > space) {
        setArr(temp.slice((space - 2) * -1));
      } else setArr(tiles);
    }
  }, [w, tiles]);

  const checkIsActiveDominoReverse = () => {
    if (playerChoice?.tile && tiles && wasm) {
      const lastTile = tiles.length > 0 ? tiles[tiles.length - 1] : wasm.startTile;

      return lastTile[1] === playerChoice.tile[0] ? false : lastTile[1] === playerChoice.tile[1];
    } else return false;
  };

  const checkIsRowDominoReverse = (tile: DominoTileType, i: number, tiles: DominoTileType[]) => {
    if (wasm) {
      const lastTile = tiles.length > 0 ? (i > 0 ? tiles[i - 1] : false) : wasm.startTile;
      // console.log({ tile, lastTile });
      return lastTile ? (lastTile[1] === tile[0] ? false : lastTile[1] === tile[1]) : false;
    } else return false;
  };

  useEffect(() => {
    if (playerChoice?.tile && tiles && wasm && !train && (active || isUserTrain)) {
      setIsDisabled(
        !isPartialSubset([tiles.length > 0 ? tiles[tiles.length - 1][1] : wasm.startTile[1]], playerChoice.tile),
      );
    } else {
      setIsDisabled(false);
    }
  }, [active, isUserTrain, playerChoice, tiles, train, wasm]);

  return (
    <div
      className={clsx(
        'relative grid grid-cols-[170px_1fr] grid-rows-[36px] gap-10 py-3 px-3.5 rounded-lg',
        'before:absolute before:inset-0 before:rounded-lg',
        active
          ? 'before:bg-gradient-to-r before:from-[white_-3%] before:to-[transparent_24.7%] before:opacity-50'
          : 'before:bg-[#EBF1EE] before:bg-opacity-90',
        getBgColors(index).backdrop,
      )}>
      <div className="relative grid grid-cols-[44px_1fr] items-center gap-3">
        {tiles && tiles.length > 0 && (
          <span className="absolute top-1/2 right-0 -translate-y-1/2 font-kanit font-bold w-6 h-6 bg-white/50 text-center rounded-full">
            {tiles?.length}
          </span>
        )}
        {train && (
          <Icon
            name="train"
            width={43}
            height={35}
            className={clsx('w-full h-auto', train ? 'text-[#FFCE4A]' : getBgColors(index).train)}
          />
        )}
        {isUserTrain &&
          (account?.decodedAddress === wasm?.players[index] && !isDisabled && playerChoice?.tile_id !== undefined ? (
            <PlayerTrain index={index} />
          ) : (
            <Icon
              name="train"
              width={43}
              height={35}
              className={clsx('w-full h-auto', train ? 'text-[#FFCE4A]' : getBgColors(index).train)}
            />
          ))}
        <h3
          className={clsx(
            'uppercase leading-4 font-semibold tracking-[0.03em] w-min min-w-[60px]',
            active && !getBgColors(index).isLight && 'text-white',
            !isUserTrain && 'col-span-2',
          )}>
          <TooltipWrapper
            text={wasm?.players[index][1] || ''}
            className="after:text-dark-500 after:!bg-primary after:!transition-none after:!shadow-md">
            <span className="line-clamp-2">{train ? 'Tequila Train' : `${wasm?.players[index][1]}`}</span>
          </TooltipWrapper>
        </h3>
      </div>

      <div className="relative flex overflow-auto max-w-full" ref={ref}>
        <div className="flex items-center gap-0.5 ">
          {arr &&
            arr.map((tile, i) => (
              <DominoItem row tile={tile} key={i} reverse={checkIsRowDominoReverse(tile, i, arr)} />
            ))}

          {(active || isUserTrain) && isAllowed && (
            <DominoZone
              id={index}
              light={active && !getBgColors(index).isLight}
              disabled={isDisabled}
              reverse={checkIsActiveDominoReverse()}
            />
          )}
        </div>
      </div>
    </div>
  );
};
