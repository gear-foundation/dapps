import clsx from 'clsx';
// import { Modal } from '@gear-js/ui';
import { findTile, getBgColors } from '../../../../app/utils';
import { Icon } from '../../../ui/icon';
import { DominoItem } from '../../../common/domino-item';

import mockData from '../mock/mock-data.json';

export const MockGameSection = () => {
  const gameState = mockData;

  const stateStartTile = gameState.startTile || 0;
  const startTile = stateStartTile && findTile(stateStartTile, gameState.tiles as any);

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
        {gameState.tracks.map((_p, i) => {
          return (
            <li key={i}>
              <div
                className={clsx(
                  'relative grid grid-cols-[170px_1fr] grid-rows-[36px] gap-10 py-3 px-3.5 rounded-lg',
                  'before:absolute before:inset-0 before:rounded-lg',
                  'before:bg-[#EBF1EE] before:bg-opacity-90',
                  getBgColors(i).backdrop,
                )}>
                <div className="relative grid grid-cols-[44px_1fr] items-center gap-3">
                  <Icon name="train" width={43} height={35} className="w-full h-auto text-[#FFCE4A]" />
                  <span className="uppercase leading-4 font-semibold tracking-[0.03em] w-min">Tequila Train</span>
                </div>
              </div>
            </li>
          );
        })}
      </ul>
      <div className="grid gap-4 mt-auto">
        <ul className="flex gap-4 justify-center">
          {gameState.players.map((_p, index) => (
            <li key={index}>
              <div className="relative flex flex-col h-full max-w-[160px]">
                <div
                  className={clsx(
                    'absolute -z-1 bottom-0 -inset-x-px h-[calc(100%+1rem)] -mt-1 bg-[#D6FE51] border-x border-dark-500/15',
                    'before:absolute before:top-0 before:right-full before:w-10 before:h-10 before:bg-[#ebf1ee] before:rounded-tr-3xl before:border-r before:border-t before:border-dark-500/15',
                    'after:absolute after:top-0 after:left-full after:w-10 after:h-10 after:bg-[#ebf1ee] after:rounded-tl-3xl after:border-l after:border-t after:border-dark-500/15',
                  )}>
                  <div
                    className={clsx(
                      'before:absolute before:top-0 before:right-full before:-z-1 before:w-10 before:h-10 before:bg-[#D6FE51]',
                      'after:absolute after:top-0 after:left-full after:-z-1 after:w-10 after:h-10 after:bg-[#D6FE51]',
                    )}
                  />
                </div>
                <div className="grow flex rounded-t-2xl bg-[#D6FE51] py-3.5 px-2.5 font-medium text-center">
                  <span className="line-clamp-2 w-full">{gameState?.players[index].name}</span>
                </div>
                <div
                  className={clsx(
                    'grow-0 flex items-center py-3.5 px-2.5',
                    getBgColors(index).backdrop,
                    getBgColors(index).isLight ? 'text-dark-500' : 'text-white',
                  )}>
                  <div className="flex flex-wrap gap-1 items-center justify-center">
                    <div
                      className={clsx(
                        'inline-flex items-center justify-center w-6 h-6 rounded-full',
                        getBgColors(index).isLight ? 'bg-black/10' : 'bg-white/15',
                      )}>
                      <Icon name="on-hands" className="m-auto w-3 h-3" />
                    </div>
                    <span className="basis-full text-center whitespace-nowrap text-[8px] leading-3">On hands</span>
                  </div>
                  <div className="flex flex-wrap gap-1 items-center justify-center">
                    <div
                      className={clsx(
                        'inline-flex items-center justify-center w-6 h-6 rounded-full',
                        getBgColors(index).isLight ? 'bg-black/10' : 'bg-white/15',
                      )}>
                      <Icon name="shots" className="m-auto w-4 h-4" />
                    </div>
                    <span className="font-bold text-lg">{gameState?.shots[index]}</span>
                    <span className="basis-full text-center whitespace-nowrap text-[8px] leading-3">
                      Number of shots
                    </span>
                  </div>
                </div>
              </div>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};
