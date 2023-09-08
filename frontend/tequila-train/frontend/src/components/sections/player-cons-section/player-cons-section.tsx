import { useApp, useGame } from 'app/context';
import { useGameMessage } from 'app/hooks/use-game';
import { PlayerDomino } from '../../common/player-domino';
import { DominoTileType } from 'app/types/game';
import { cn, getTileId } from 'app/utils';
import { useEffect, useState } from 'react';

export const PlayerConsSection = () => {
  const { setIsPending, isPending, setOpenEmptyPopup } = useApp();
  const { game, gameWasm: wasm, setSelectedDomino, selectedDomino, setPlayerChoice, playerChoice } = useGame();
  const handleMessage = useGameMessage();
  const [turnPending, setTurnPending] = useState(false);
  const [passPending, setPassPending] = useState(false);

  useEffect(() => {
    return () => {
      setPlayerChoice(undefined);
      setSelectedDomino(undefined);
    };
  }, []);

  const onSuccess = () => {
    setTurnPending(false);
    setPassPending(false);
    setIsPending(false);
  };
  const onError = () => {
    setTurnPending(false);
    setPassPending(false);
    setIsPending(false);
  };

  const onSelect = ([i, tile]: [number, DominoTileType]) => {
    if (selectedDomino) {
      selectedDomino[0] !== i ? setSelectedDomino([i, tile]) : setSelectedDomino(undefined);
    } else {
      setSelectedDomino([i, tile]);
    }

    if (game?.gameState) {
      if (playerChoice) {
        playerChoice.tile !== tile
          ? setPlayerChoice({ ...playerChoice, tile, tile_id: getTileId(tile, game.gameState?.tiles).toString() })
          : setPlayerChoice({
              ...playerChoice,
              tile: undefined,
              tile_id: undefined,
            });
      } else {
        setPlayerChoice({ tile, tile_id: getTileId(tile, game.gameState?.tiles).toString() });
      }
    }
  };

  const onTurn = () => {
    if (playerChoice?.track_id !== undefined && playerChoice.tile_id !== undefined) {
      const { tile_id, track_id, remove_train } = playerChoice;

      if (+track_id >= 0 && +tile_id >= 0) {
        setIsPending((prev) => !prev);
        setTurnPending(true);
        handleMessage({ Place: { tile_id, track_id, remove_train } }, { onSuccess, onError });
      }
    } else {
      setOpenEmptyPopup(true);
    }
  };

  const onPass = () => {
    setIsPending((prev) => !prev);
    setPassPending(true);
    handleMessage({ Skip: null }, { onSuccess, onError });
  };

  return (
    <div className="relative flex justify-between bg-[#D6FE51] py-3 px-7 rounded-2xl before:absolute before:-inset-px before:-z-1 before:rounded-[17px] before:border before:border-dark-500/15">
      <div className="flex flex-wrap items-center gap-2 min-h-[72px]">
        {wasm &&
          wasm.playersTiles[+wasm.currentPlayer].map((tile, i) => (
            <PlayerDomino
              tile={tile}
              key={i}
              onClick={() => onSelect([i, tile])}
              isSelected={selectedDomino && i === selectedDomino[0]}
            />
          ))}
      </div>
      <div className="py-1 border-l border-primary pl-6 flex flex-col gap-3 min-w-[175px]">
        <button
          className={cn('btn btn--primary text-dark-500 py-1.5', turnPending && 'btn--loading')}
          onClick={onTurn}
          disabled={isPending}>
          Turn
        </button>
        <button
          className={cn('btn btn--black my-auto py-1.5', passPending && 'btn--loading')}
          onClick={onPass}
          disabled={isPending}>
          Pass
        </button>
      </div>
    </div>
  );
};
