import { ActorId } from 'sails-js';

declare global {
  export interface Config {
    s_per_block: number | string | bigint;
    gas_to_remove_game: number | string | bigint;
    gas_to_delete_session: number | string | bigint;
    time_interval: number;
    turn_deadline_ms: number | string | bigint;
    minimum_session_duration_ms: number | string | bigint;
  }

  export interface SignatureData {
    key: ActorId;
    duration: number | string | bigint;
    allowed_actions: Array<ActionsForSession>;
  }

  export type ActionsForSession = 'StartGame' | 'Move' | 'Skip';

  export interface SessionData {
    key: ActorId;
    expires: number | string | bigint;
    allowed_actions: Array<ActionsForSession>;
    expires_at_block: number;
  }

  export interface GameInstance {
    board: Array<Mark | null>;
    player_mark: Mark;
    bot_mark: Mark;
    last_time: number | string | bigint;
    game_over: boolean;
    game_result: GameResult | null;
  }

  export type Mark = 'X' | 'O';

  export type GameResult = 'Player' | 'Bot' | 'Draw';
}
