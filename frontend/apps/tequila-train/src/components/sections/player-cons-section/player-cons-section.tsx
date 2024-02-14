import { useApp, useGame } from 'app/context';
import { useGameMessage } from 'app/hooks/use-game';
import { PlayerDomino } from '../../common/player-domino';
import { DominoTileType } from 'app/types/game';
import { cn, findTile, getTileId } from 'app/utils';
import { useEffect, useState } from 'react';
import { Icon } from 'components/ui/icon';
import { useAccount } from '@gear-js/react-hooks';

export const PlayerConsSection = () => {
  const { account } = useAccount();
  const { setIsPending, isPending } = useApp();
  const { game, setSelectedDomino, selectedDomino, setPlayerChoice, playerChoice } = useGame();
  const [playersTiles, setPlayersTiles] = useState<DominoTileType[]>([])
  const handleMessage = useGameMessage();
  const [turnPending, setTurnPending] = useState(false);
  const [passPending, setPassPending] = useState(false);

  useEffect(() => {
    return () => {
      setPlayerChoice(undefined);
      setSelectedDomino(undefined);
    };
  }, []);

  useEffect(() => {
    if (game) {
      const playersTiles = Object.entries(game.gameState.tileToPlayer)
        .filter(([key, value]) => value === game.gameState.currentPlayer)
        .map(([key, value]) => findTile(key, game?.gameState?.tiles))
        .filter(tile => tile !== null) as DominoTileType[];

      setPlayersTiles(playersTiles)
    }

  }, [game])

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
    let newPlayerChoice;

    if (game) {
      if (selectedDomino) {
        if (selectedDomino[0] !== i) {
          setSelectedDomino([i, tile]);
          newPlayerChoice = { ...playerChoice, tile, tile_id: getTileId(tile, game.gameState?.tiles).toString() };
        } else {
          setSelectedDomino(undefined);
          newPlayerChoice = { ...playerChoice, tile: undefined, tile_id: undefined };
        }
      } else {
        setSelectedDomino([i, tile]);
        newPlayerChoice = { tile, tile_id: getTileId(tile, game.gameState?.tiles).toString() };
      }
    }
    setPlayerChoice(newPlayerChoice);
  };

  const onTurn = () => {
    if (playerChoice?.track_id !== undefined && playerChoice.tile_id !== undefined) {
      const { tile_id, track_id, remove_train } = playerChoice;

      if (+track_id >= 0 && +tile_id >= 0) {
        setIsPending((prev) => !prev);
        setTurnPending(true);
        handleMessage({ payload: { Place: { creator: game?.admin, tile_id, track_id, remove_train } }, onSuccess, onError });
      }
    }
  };

  const onPass = () => {
    setIsPending((prev) => !prev);
    setPassPending(true);
    handleMessage({ payload: { Skip: { creator: game?.admin } }, onSuccess, onError });
  };

  const isDisabledShot = () => {
    if (playerChoice) {
      const { tile_id, track_id } = playerChoice;

      const validTrack = track_id === game?.gameState.currentPlayer;
      const validChoice = tile_id !== undefined && track_id !== undefined && +track_id >= 0 && +tile_id >= 0;
      const isCurrentPlayer = account?.decodedAddress === game?.gameState?.players[+game?.gameState.currentPlayer].id;
      const tracks = game?.gameState.tracks[+game.gameState.currentPlayer];
      const isTracksValid = tracks?.hasTrain && tracks.tiles.length > 0;

      return isPending || !validChoice || !validTrack || !isCurrentPlayer || !isTracksValid
    } else {
      return true
    }
  }

  const isDisabledTurn = () => {
    if (playerChoice) {
      const { tile_id, track_id } = playerChoice;

      return isPending || !tile_id || !track_id
    } else {
      return true
    }
  }

  const onShot = () => {
    if (playerChoice) {
      const { tile_id, track_id } = playerChoice;

      if (track_id === game?.gameState.currentPlayer) {
        setPlayerChoice((prev) => ({ ...prev, remove_train: true }));
        setIsPending((prev) => !prev);
        setTurnPending(true);
        handleMessage({
          payload: {
            Place: {
              creator: game?.admin,
              tile_id,
              track_id,
              remove_train: true
            }
          },
          onSuccess,
          onError
        });
      }
    }
  };

  return (
    <div className="relative flex justify-between bg-[#D6FE51] py-3 px-7 rounded-2xl before:absolute before:-inset-px before:-z-1 before:rounded-[17px] before:border before:border-dark-500/15">
      <div className="flex flex-wrap items-center gap-2 min-h-[72px]">
        {playersTiles &&
          playersTiles.map((tile, i) => (
            <PlayerDomino
              tile={tile}
              key={i}
              onClick={() => onSelect([i, tile])}
              isSelected={selectedDomino && i === selectedDomino[0]}
            />
          ))}
      </div>
      <div className="py-1 border-l border-primary pl-6 flex flex-col gap-3 min-w-[175px]">
        <div className="flex items-center gap-3">
          <button
            className={cn('btn btn--primary bg-[#67CB4D] text-dark-500 py-2 gap-2', turnPending && 'btn--loading')}
            onClick={onTurn}
            disabled={isDisabledTurn()}>
            <Icon name="domino" width={15} height={15} />
            Draw
          </button>
          <button
            className={cn('btn btn--primary bg-[#FFCE4A] text-dark-500 py-2 gap-2', turnPending && 'btn--loading')}
            onClick={onShot}
            disabled={isDisabledShot()}>
            <Icon name="shot" width={15} height={15} />
            Shot
          </button>
        </div>

        <button
          className={cn('btn btn--primary bg-[#353535] text-white py-2 w-full', turnPending && 'btn--loading')}
          onClick={onPass}
          disabled={isPending}
        >
          Pass
        </button>
      </div>
    </div>
  );
};
