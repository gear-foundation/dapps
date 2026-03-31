/* eslint-disable */
import { GearApi } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, QueryBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

type ActorId = string;

export interface InitConfig {
  config: Config;
}

export interface Config {
  gas_to_remove_game: number | string | bigint;
  gas_to_delete_session: number | string | bigint;
  initial_speed: number;
  min_speed: number;
  max_speed: number;
  gas_for_round: number | string | bigint;
  time_interval: number;
  max_distance: number;
  time: number;
  time_for_game_storage: number | string | bigint;
  block_duration_ms: number | string | bigint;
  gas_for_reply_deposit: number | string | bigint;
  minimum_session_duration_ms: number | string | bigint;
  s_per_block: number | string | bigint;
}

export type StrategyAction = 'BuyAcceleration' | 'BuyShell' | 'Skip';

export interface Game {
  cars: Record<ActorId, Car>;
  car_ids: Array<ActorId>;
  current_turn: number;
  state: GameState;
  result: GameResult | null;
  current_round: number;
  last_time_step: number | string | bigint;
}

export interface Car {
  position: number;
  speed: number;
  car_actions: Array<RoundAction>;
  round_result: RoundAction | null;
}

export type RoundAction = 'Accelerated' | 'SlowedDown' | 'SlowedDownAndAccelerated';

export type GameState = 'ReadyToStart' | 'Race' | 'Stopped' | 'Finished' | 'PlayerAction';

export type GameResult = 'Win' | 'Draw' | 'Lose';

export interface RoundInfo {
  cars: Array<[ActorId, number, RoundAction | null]>;
  result: GameResult | null;
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

export class Program {
  public readonly registry: TypeRegistry;
  public readonly carRacesService: CarRacesService;
  public readonly session: Session;

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      InitConfig: { config: 'Config' },
      Config: {
        gas_to_remove_game: 'u64',
        gas_to_delete_session: 'u64',
        initial_speed: 'u32',
        min_speed: 'u32',
        max_speed: 'u32',
        gas_for_round: 'u64',
        time_interval: 'u32',
        max_distance: 'u32',
        time: 'u32',
        time_for_game_storage: 'u64',
        block_duration_ms: 'u64',
        gas_for_reply_deposit: 'u64',
        minimum_session_duration_ms: 'u64',
        s_per_block: 'u64',
      },
      StrategyAction: { _enum: ['BuyAcceleration', 'BuyShell', 'Skip'] },
      Game: {
        cars: 'BTreeMap<[u8;32], Car>',
        car_ids: 'Vec<[u8;32]>',
        current_turn: 'u8',
        state: 'GameState',
        result: 'Option<GameResult>',
        current_round: 'u32',
        last_time_step: 'u64',
      },
      Car: { position: 'u32', speed: 'u32', car_actions: 'Vec<RoundAction>', round_result: 'Option<RoundAction>' },
      RoundAction: { _enum: ['Accelerated', 'SlowedDown', 'SlowedDownAndAccelerated'] },
      GameState: { _enum: ['ReadyToStart', 'Race', 'Stopped', 'Finished', 'PlayerAction'] },
      GameResult: { _enum: ['Win', 'Draw', 'Lose'] },
      RoundInfo: { cars: 'Vec<([u8;32], u32, Option<RoundAction>)>', result: 'Option<GameResult>' },
      SignatureData: { key: '[u8;32]', duration: 'u64', allowed_actions: 'Vec<ActionsForSession>' },
      ActionsForSession: { _enum: ['StartGame', 'Move', 'Skip'] },
      SessionData: {
        key: '[u8;32]',
        expires: 'u64',
        allowed_actions: 'Vec<ActionsForSession>',
        expires_at_block: 'u32',
      },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.carRacesService = new CarRacesService(this);
    this.session = new Session(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, init_config: InitConfig): TransactionBuilder<null> {
    // @ts-ignore
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      init_config,
      'InitConfig',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, init_config: InitConfig) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      null,
      'New',
      init_config,
      'InitConfig',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class CarRacesService {
  constructor(private _program: Program) {}

  public addAdmin(admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'CarRacesService',
      'AddAdmin',
      admin,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public addStrategyIds(car_ids: Array<ActorId>): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'CarRacesService',
      'AddStrategyIds',
      car_ids,
      'Vec<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  public allowMessages(messages_allowed: boolean): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'CarRacesService',
      'AllowMessages',
      messages_allowed,
      'bool',
      'Null',
      this._program.programId,
    );
  }

  public playerMove(strategy_move: StrategyAction, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'CarRacesService',
      'PlayerMove',
      [strategy_move, session_for_account],
      '(StrategyAction, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public removeAdmin(admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'CarRacesService',
      'RemoveAdmin',
      admin,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public removeGameInstance(account: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'CarRacesService',
      'RemoveGameInstance',
      account,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public removeInstances(player_ids: Array<ActorId> | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'CarRacesService',
      'RemoveInstances',
      player_ids,
      'Option<Vec<[u8;32]>>',
      'Null',
      this._program.programId,
    );
  }

  public startGame(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'CarRacesService',
      'StartGame',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  public updateConfig(config: Config): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'CarRacesService',
      'UpdateConfig',
      config,
      'Config',
      'Null',
      this._program.programId,
    );
  }

  public admins(): QueryBuilder<Array<ActorId>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'CarRacesService',
      'Admins',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public allGames(): QueryBuilder<Array<[ActorId, Game]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<[ActorId, Game]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'CarRacesService',
      'AllGames',
      null,
      null,
      'Vec<([u8;32], Game)>',
    );
  }

  public configState(): QueryBuilder<Config> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Config>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'CarRacesService',
      'ConfigState',
      null,
      null,
      'Config',
    );
  }

  public game(account_id: ActorId): QueryBuilder<Game | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Game | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'CarRacesService',
      'Game',
      account_id,
      '[u8;32]',
      'Option<Game>',
    );
  }

  public messagesAllowed(): QueryBuilder<boolean> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<boolean>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'CarRacesService',
      'MessagesAllowed',
      null,
      null,
      'bool',
    );
  }

  public strategyIds(): QueryBuilder<Array<ActorId>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'CarRacesService',
      'StrategyIds',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public subscribeToRoundInfoEvent(callback: (data: RoundInfo) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'CarRacesService' && getFnNamePrefix(payload) === 'RoundInfo') {
        callback(
          this._program.registry
            .createType('(String, String, RoundInfo)', message.payload)[2]
            .toJSON() as unknown as RoundInfo,
        );
      }
    });
  }
}

export class Session {
  constructor(private _program: Program) {}

  public createSession(signature_data: SignatureData, signature: `0x${string}` | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Session',
      'CreateSession',
      [signature_data, signature],
      '(SignatureData, Option<Vec<u8>>)',
      'Null',
      this._program.programId,
    );
  }

  public deleteSessionFromAccount(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Session',
      'DeleteSessionFromAccount',
      null,
      null,
      'Null',
      this._program.programId,
    );
  }

  public deleteSessionFromProgram(session_for_account: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Session',
      'DeleteSessionFromProgram',
      session_for_account,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public sessionForTheAccount(account: ActorId): QueryBuilder<SessionData | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<SessionData | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Session',
      'SessionForTheAccount',
      account,
      '[u8;32]',
      'Option<SessionData>',
    );
  }

  public sessions(): QueryBuilder<Array<[ActorId, SessionData]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<[ActorId, SessionData]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Session',
      'Sessions',
      null,
      null,
      'Vec<([u8;32], SessionData)>',
    );
  }

  public subscribeToSessionCreatedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Session' && getFnNamePrefix(payload) === 'SessionCreated') {
        callback(null);
      }
    });
  }

  public subscribeToSessionDeletedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Session' && getFnNamePrefix(payload) === 'SessionDeleted') {
        callback(null);
      }
    });
  }
}
