import { ActorId } from 'sails-js';

declare global {
  export interface Config {
    one_point_in_value: number | string | bigint;
    max_number_gold_coins: number;
    max_number_silver_coins: number;
    points_per_gold_coin_easy: number | string | bigint;
    points_per_silver_coin_easy: number | string | bigint;
    points_per_gold_coin_medium: number | string | bigint;
    points_per_silver_coin_medium: number | string | bigint;
    points_per_gold_coin_hard: number | string | bigint;
    points_per_silver_coin_hard: number | string | bigint;
    gas_for_finish_tournament: number | string | bigint;
    gas_for_mint_fungible_token: number | string | bigint;
    gas_to_delete_session: number | string | bigint;
    minimum_session_duration_ms: number | string | bigint;
    s_per_block: number | string | bigint;
  }

  export interface SignatureData {
    key: ActorId;
    duration: number | string | bigint;
    allowed_actions: Array<ActionsForSession>;
  }

  export type ActionsForSession =
    | 'CreateNewTournament'
    | 'RegisterForTournament'
    | 'CancelRegister'
    | 'CancelTournament'
    | 'DeletePlayer'
    | 'FinishSingleGame'
    | 'StartTournament'
    | 'RecordTournamentResult'
    | 'LeaveGame';

  export interface SessionData {
    key: ActorId;
    expires: number | string | bigint;
    allowed_actions: Array<ActionsForSession>;
    expires_at_block: number;
  }

  export type Status =
    | { paused: null }
    | { startedUnrewarded: null }
    | { startedWithFungibleToken: { ft_address: ActorId } }
    | { startedWithNativeToken: null };

  export type Level = 'Easy' | 'Medium' | 'Hard';

  export interface VaraManState {
    tournaments: Array<[ActorId, TournamentState]>;
    players_to_game_id: Array<[ActorId, ActorId]>;
    status: Status;
    config: Config;
    admins: Array<ActorId>;
    dns_info: [ActorId, string] | null;
  }

  export interface TournamentState {
    tournament_name: string;
    admin: ActorId;
    level: Level;
    participants: Array<[ActorId, Player]>;
    bid: number | string | bigint;
    stage: Stage;
    duration_ms: number;
  }

  export interface Player {
    name: string;
    time: number | string | bigint;
    points: number | string | bigint;
  }

  export type Stage = { registration: null } | { started: number | string | bigint } | { finished: Array<ActorId> };
}
