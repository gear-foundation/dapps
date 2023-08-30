import { Hex } from '@gear-js/api';

export const payloads = {
  init: function(
    bet_size: number,
    lobby_players: [Hex],
  ) {
    return {
      bet_size,
      lobby_players,
    }
  },
  addPlayerInLobby: function(player: Hex) {
    return {
      AddPlayerInLobby: player,
    }
  },
  removePlayerFromLobby: function(player: Hex) {
    return {
      RemovePlayerFromLobby: {
        player,
      }
    }
  },
  setLobbyPlayersList: function(list: [Hex]) {
    return {
      SetLobbyPlayersList: list, 
    }
  },
  setBetSize: function(bet_size: number) {
    return {
      SetBetSize: bet_size,
    }
  },
  makeMove: function(hashed_move: string) {
    return {
      MakeMove: hashed_move
    }
  },
  reveal: function(raw_move: string) {
    return {
      Reveal: raw_move,
    }
  },
  stopGame: 'StopGame',
  betSizeState: 'BetSize',
  lobbyListState: 'LobbyList',
  gameState: 'GameState',
};