import { PlayerTrackSection } from '../player-track-section';
import { PlayerCardSection } from '../player-card-section';
import { PlayerConsSection } from '../player-cons-section';
import { useApp, useGame } from 'app/context';
import { SelectDominoPopup } from '../../popups/select-domino-popup/select-domino-popup';
import clsx from 'clsx';
import { getBgColors } from '../../../app/utils';
import { Icon } from '../../ui/icon';
import { DominoItem } from '../../common/domino-item';
import { WinnerPopup } from '../../popups/winner-popup/winner-popup';
import * as React from 'react';

export const GameSection = () => {
  const { isAllowed, openEmptyPopup, setOpenEmptyPopup, setOpenWinnerPopup, openWinnerPopup } = useApp();
  const { gameWasm: state, players } = useGame();

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
              <div className="flex items-center gap-0.5 ">{state && <DominoItem row tile={state.startTile} />}</div>
            </div>
          </div>
        </li>
        {state?.tracks.map((p, i) => (
          <li key={i}>
            <PlayerTrackSection
              index={i}
              isUserTrain={p.hasTrain}
              active={+state?.currentPlayer === i}
              tiles={p.tiles}
            />
          </li>
        ))}
      </ul>
      <div className="grid gap-4 mt-auto">
        {isAllowed && <PlayerConsSection />}

        <ul className="flex gap-4 justify-center">
          {players.map((p, i) => (
            <li key={i}>
              <PlayerCardSection index={i} active={isAllowed && Boolean(state && +state.currentPlayer === i)} />
            </li>
          ))}
        </ul>
      </div>

      <WinnerPopup isOpen={openWinnerPopup} setIsOpen={setOpenWinnerPopup} />
      <SelectDominoPopup isOpen={openEmptyPopup} setIsOpen={setOpenEmptyPopup} />
    </div>
  );
};
