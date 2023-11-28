import { Hex } from '@gear-js/api';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { LOCAL_STORAGE } from 'consts';
import { GameStageType, StageType, StateWinnerType } from 'types';
import { isExists, isMinValue } from './form';
import { onSubmitReveal, onSubmitMove, onClickRegister } from './sendMessage';

const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

const handleRouteChange = (admin: boolean | undefined, routeChange: (arg: string) => void) => {
  if (admin) {
    routeChange('lobby admin');
  } else {
    routeChange('game');
  }
};

const gameStageFinishedPlayers = (gameStage: GameStageType) => {
  let finishedPlayers: Hex[] = [];
  if (gameStage?.InProgress?.finishedPlayers.length) finishedPlayers = gameStage?.InProgress?.finishedPlayers;
  if (gameStage?.Reveal?.finishedPlayers.length) finishedPlayers = gameStage?.Reveal?.finishedPlayers;
  return { finishedPlayers };
};

const getGameStageText = (stageData: string | { inProgress: {} | { Reveal: {} } }) => {
  const gameStageKeys = typeof stageData === 'object' ? Object.keys(stageData) : [];
  let gameStageText: StageType = 'preparation';
  if (gameStageKeys?.includes('InProgress')) {
    gameStageText = 'progress';
  } else if (gameStageKeys?.includes('Reveal')) {
    gameStageText = 'reveal';
  } else {
    gameStageText = 'preparation';
  }
  return { gameStageText };
};

const getLoosers = (prevLobbyList: Hex[], lobbyList: Hex[] | undefined, winnerState: StateWinnerType) => {
  const loosers = [] as Hex[];
  prevLobbyList.forEach((prevLobbyPlayer) => {
    if (!lobbyList?.includes(prevLobbyPlayer as never) && prevLobbyPlayer !== winnerState.Winner) {
      loosers.push(prevLobbyPlayer);
    }
  });
  return { loosers };
};

const getButtonVisible = (stage: StageType, finishedAccount: boolean) => {
  const buttonVisible = (stage as StageType) !== 'preparation' && !finishedAccount;
  return { buttonVisible };
};

export {
  isLoggedIn,
  isExists,
  isMinValue,
  onSubmitReveal,
  onSubmitMove,
  onClickRegister,
  handleRouteChange,
  gameStageFinishedPlayers,
  getGameStageText,
  getLoosers,
  getButtonVisible,
};
