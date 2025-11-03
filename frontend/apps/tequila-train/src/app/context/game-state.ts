import { useState } from 'react';

import { DominoTileType, GameType, IState, PlayerChoiceType, PlayersGame } from '../types/game';

function useGameState() {
  const [state, setState] = useState<IState>();
  const [game, setGame] = useState<GameType | null>(null);
  const [timer, setTimer] = useState(0);
  const [players, setPlayers] = useState<PlayersGame[]>([]);
  const [isAdmin, setIsAdmin] = useState(false);
  const [selectedDomino, setSelectedDomino] = useState<[number, DominoTileType]>();
  const [playerTiles, setPlayerTiles] = useState<DominoTileType[]>();
  const [playerChoice, setPlayerChoice] = useState<PlayerChoiceType>();
  const [previousGame, setPreviousGame] = useState<GameType | null>(null);

  return {
    state,
    setState,
    game,
    setGame,
    timer,
    setTimer,
    players,
    setPlayers,
    isAdmin,
    setIsAdmin,
    playerTiles,
    setPlayerTiles,
    selectedDomino,
    setSelectedDomino,
    playerChoice,
    setPlayerChoice,
    previousGame,
    setPreviousGame,
  };
}

type GameContextValue = ReturnType<typeof useGameState>;

export { useGameState };
export type { GameContextValue };
