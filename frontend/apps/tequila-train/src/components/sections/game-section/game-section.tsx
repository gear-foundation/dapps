import { PlayerTrackSection } from '../player-track-section';
import { PlayerCardSection } from '../player-card-section';
import { PlayerConsSection } from '../player-cons-section';
import { useApp, useGame } from '@/app/context';
import clsx from 'clsx';
import { convertFormattedTileToNumbers, findTile, getBgColors } from '../../../app/utils';
import { Icon } from '../../ui/icon';
import { DominoItem } from '../../common/domino-item';
import { CanceledSection } from './canceled-modal';
import { FinishedSection } from './finished-modal';
import { useEffect } from 'react';

export const GameSection = () => {
  const { isAllowed, openWinnerPopup, setOpenWinnerPopup, openEmptyPopup } = useApp();
  const { game: state, players } = useGame();

  useEffect(() => {
    if (state && state.state.Winners) {
      setOpenWinnerPopup(true);
    } else {
      setOpenWinnerPopup(false);
    }
  }, [state]);

  const stateStartTile = state?.gameState?.startTile;
  const startTile = state && stateStartTile && findTile(stateStartTile, state.gameState.tiles);

  return (
    <div className="container-xl flex flex-col grow">
      <ul className="space-y-px">
        <li>
          <div
            className={clsx(
              'relative grid grid-cols-[170px_1fr] grid-rows-[36px] gap-10 py-3 px-3.5 rounded-lg',
              'before:absolute before:inset-0 before:rounded-lg',
              'before:bg-[#EBF1EE] before:bg-opacity-90',
              getBgColors(-1).backdrop,
            )}>
            <div className="relative grid grid-cols-[44px_1fr] items-center gap-3">
              <Icon name="train" width={43} height={35} className="w-full h-auto text-[#FFCE4A]" />
              <span className="uppercase leading-4 font-semibold tracking-[0.03em] w-min">Tequila Train</span>
            </div>

            <div className="relative flex overflow-auto max-w-full">
              <div className="flex items-center gap-0.5 ">{startTile && <DominoItem row tile={startTile} />}</div>
            </div>
          </div>
        </li>
        {state?.gameState?.tracks.map((p, i) => {
          const tiles = p.tiles.map((t) => {
            const tileId = convertFormattedTileToNumbers(t);

            return tileId;
          });

          return (
            <li key={i}>
              <PlayerTrackSection
                index={i}
                isUserTrain={p.hasTrain}
                active={+state?.gameState?.currentPlayer === i}
                tiles={tiles}
              />
            </li>
          );
        })}
      </ul>
      <div className="grid gap-4 mt-auto">
        {isAllowed && <PlayerConsSection />}

        <ul className="flex gap-4 justify-center">
          {players.map((_p, i) => (
            <li key={i}>
              <PlayerCardSection
                index={i}
                active={isAllowed && Boolean(state && +state.gameState.currentPlayer === i)}
              />
            </li>
          ))}
        </ul>
      </div>

      {openEmptyPopup && <CanceledSection />}
      {openWinnerPopup && <FinishedSection />}
    </div>
  );
};
