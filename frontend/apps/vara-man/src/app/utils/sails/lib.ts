/* eslint-disable */

import { GearApi, BaseGearProgram, HexString } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import {
  TransactionBuilder,
  ActorId,
  QueryBuilder,
  getServiceNamePrefix,
  getFnNamePrefix,
  ZERO_ADDRESS,
} from 'sails-js';

export class SailsProgram {
  public readonly registry: TypeRegistry;
  public readonly session: Session;
  public readonly varaMan: VaraMan;
  private _program?: BaseGearProgram;

  constructor(
    public api: GearApi,
    programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      Config: {
        one_point_in_value: 'u128',
        max_number_gold_coins: 'u16',
        max_number_silver_coins: 'u16',
        points_per_gold_coin_easy: 'u128',
        points_per_silver_coin_easy: 'u128',
        points_per_gold_coin_medium: 'u128',
        points_per_silver_coin_medium: 'u128',
        points_per_gold_coin_hard: 'u128',
        points_per_silver_coin_hard: 'u128',
        gas_for_finish_tournament: 'u64',
        gas_for_mint_fungible_token: 'u64',
        gas_to_delete_session: 'u64',
        minimum_session_duration_ms: 'u64',
        s_per_block: 'u64',
      },
      SignatureData: { key: '[u8;32]', duration: 'u64', allowed_actions: 'Vec<ActionsForSession>' },
      ActionsForSession: {
        _enum: [
          'CreateNewTournament',
          'RegisterForTournament',
          'CancelRegister',
          'CancelTournament',
          'DeletePlayer',
          'FinishSingleGame',
          'StartTournament',
          'RecordTournamentResult',
          'LeaveGame',
        ],
      },
      SessionData: {
        key: '[u8;32]',
        expires: 'u64',
        allowed_actions: 'Vec<ActionsForSession>',
        expires_at_block: 'u32',
      },
      Status: {
        _enum: {
          Paused: 'Null',
          StartedUnrewarded: 'Null',
          StartedWithFungibleToken: { ft_address: '[u8;32]' },
          StartedWithNativeToken: 'Null',
        },
      },
      Level: { _enum: ['Easy', 'Medium', 'Hard'] },
      VaraManState: {
        tournaments: 'Vec<([u8;32], TournamentState)>',
        players_to_game_id: 'Vec<([u8;32], [u8;32])>',
        status: 'Status',
        config: 'Config',
        admins: 'Vec<[u8;32]>',
        dns_info: 'Option<([u8;32], String)>',
      },
      TournamentState: {
        tournament_name: 'String',
        admin: '[u8;32]',
        level: 'Level',
        participants: 'Vec<([u8;32], Player)>',
        bid: 'u128',
        stage: 'Stage',
        duration_ms: 'u32',
      },
      Player: { name: 'String', time: 'u128', points: 'u128' },
      Stage: { _enum: { Registration: 'Null', Started: 'u64', Finished: 'Vec<[u8;32]>' } },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);
    if (programId) {
      this._program = new BaseGearProgram(programId, api);
    }

    this.session = new Session(this);
    this.varaMan = new VaraMan(this);
  }

  public get programId(): `0x${string}` {
    if (!this._program) throw new Error(`Program ID is not set`);
    return this._program.id;
  }

  newCtorFromCode(
    code: Uint8Array | Buffer | HexString,
    config: Config,
    dns_id_and_name: [ActorId, string] | null,
  ): TransactionBuilder<null> {
    // @ts-ignore
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      [config, dns_id_and_name],
      '(Config, Option<([u8;32], String)>)',
      'String',
      code,
      async (programId) => {
        this._program = await BaseGearProgram.new(programId, this.api);
      },
    );
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, config: Config, dns_id_and_name: [ActorId, string] | null) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      null,
      'New',
      [config, dns_id_and_name],
      '(Config, Option<([u8;32], String)>)',
      'String',
      codeId,
      async (programId) => {
        this._program = await BaseGearProgram.new(programId, this.api);
      },
    );
    return builder;
  }
}

export class Session {
  constructor(private _program: SailsProgram) {}

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

export class VaraMan {
  constructor(private _program: SailsProgram) {}

  public addAdmin(new_admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'AddAdmin',
      new_admin_id,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public cancelRegister(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'CancelRegister',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  public cancelTournament(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'CancelTournament',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  public changeConfig(config: Config): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'ChangeConfig',
      config,
      'Config',
      'Null',
      this._program.programId,
    );
  }

  public changeStatus(status: Status): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'ChangeStatus',
      status,
      'Status',
      'Null',
      this._program.programId,
    );
  }

  public createNewTournament(
    tournament_name: string,
    name: string,
    level: Level,
    duration_ms: number,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'CreateNewTournament',
      [tournament_name, name, level, duration_ms, session_for_account],
      '(String, String, Level, u32, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public deletePlayer(player_id: ActorId, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'DeletePlayer',
      [player_id, session_for_account],
      '([u8;32], Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public finishSingleGame(
    gold_coins: number,
    silver_coins: number,
    level: Level,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'FinishSingleGame',
      [gold_coins, silver_coins, level, session_for_account],
      '(u16, u16, Level, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public finishTournament(admin_id: ActorId, time_start: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'FinishTournament',
      [admin_id, time_start],
      '([u8;32], u64)',
      'Null',
      this._program.programId,
    );
  }

  public kill(inheritor: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'Kill',
      inheritor,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public leaveGame(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'LeaveGame',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  public recordTournamentResult(
    time: number | string | bigint,
    gold_coins: number,
    silver_coins: number,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'RecordTournamentResult',
      [time, gold_coins, silver_coins, session_for_account],
      '(u128, u16, u16, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public registerForTournament(
    admin_id: ActorId,
    name: string,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'RegisterForTournament',
      [admin_id, name, session_for_account],
      '([u8;32], String, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public startTournament(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'VaraMan',
      'StartTournament',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  public admins(): QueryBuilder<Array<ActorId>> {
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'VaraMan',
      'Admins',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public all(): QueryBuilder<VaraManState> {
    return new QueryBuilder<VaraManState>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'VaraMan',
      'All',
      null,
      null,
      'VaraManState',
    );
  }

  public config(): QueryBuilder<Config> {
    return new QueryBuilder<Config>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'VaraMan',
      'Config',
      null,
      null,
      'Config',
    );
  }

  public getTournament(player_id: ActorId): QueryBuilder<[TournamentState, number | string | bigint | null] | null> {
    return new QueryBuilder<[TournamentState, number | string | bigint | null] | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'VaraMan',
      'GetTournament',
      player_id,
      '[u8;32]',
      'Option<(TournamentState, Option<u64>)>',
    );
  }

  public status(): QueryBuilder<Status> {
    return new QueryBuilder<Status>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'VaraMan',
      'Status',
      null,
      null,
      'Status',
    );
  }

  public subscribeToGameFinishedEvent(
    callback: (data: {
      winners: Array<ActorId>;
      participants: Array<[ActorId, Player]>;
      prize: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'GameFinished') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"winners":"Vec<[u8;32]>","participants":"Vec<([u8;32], Player)>","prize":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            winners: Array<ActorId>;
            participants: Array<[ActorId, Player]>;
            prize: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToSingleGameFinishedEvent(
    callback: (data: {
      gold_coins: number;
      silver_coins: number;
      points: number | string | bigint;
      maximum_possible_points: number | string | bigint;
      maximum_number_gold_coins: number;
      maximum_number_silver_coins: number;
      prize: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'SingleGameFinished') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"gold_coins":"u16","silver_coins":"u16","points":"u128","maximum_possible_points":"u128","maximum_number_gold_coins":"u16","maximum_number_silver_coins":"u16","prize":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            gold_coins: number;
            silver_coins: number;
            points: number | string | bigint;
            maximum_possible_points: number | string | bigint;
            maximum_number_gold_coins: number;
            maximum_number_silver_coins: number;
            prize: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToNewTournamentCreatedEvent(
    callback: (data: {
      tournament_name: string;
      name: string;
      level: Level;
      bid: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'NewTournamentCreated') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"tournament_name":"String","name":"String","level":"Level","bid":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            tournament_name: string;
            name: string;
            level: Level;
            bid: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToPlayerRegisteredEvent(
    callback: (data: { admin_id: ActorId; name: string; bid: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'PlayerRegistered') {
        callback(
          this._program.registry
            .createType('(String, String, {"admin_id":"[u8;32]","name":"String","bid":"u128"})', message.payload)[2]
            .toJSON() as unknown as { admin_id: ActorId; name: string; bid: number | string | bigint },
        );
      }
    });
  }

  public subscribeToRegisterCanceledEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'RegisterCanceled') {
        callback(null);
      }
    });
  }

  public subscribeToTournamentCanceledEvent(
    callback: (data: { admin_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'TournamentCanceled') {
        callback(
          this._program.registry
            .createType('(String, String, {"admin_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { admin_id: ActorId },
        );
      }
    });
  }

  public subscribeToPlayerDeletedEvent(
    callback: (data: { player_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'PlayerDeleted') {
        callback(
          this._program.registry
            .createType('(String, String, {"player_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { player_id: ActorId },
        );
      }
    });
  }

  public subscribeToResultTournamentRecordedEvent(
    callback: (data: {
      gold_coins: number;
      silver_coins: number;
      time: number | string | bigint;
      points: number | string | bigint;
      maximum_possible_points: number | string | bigint;
      maximum_number_gold_coins: number;
      maximum_number_silver_coins: number;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'ResultTournamentRecorded') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"gold_coins":"u16","silver_coins":"u16","time":"u128","points":"u128","maximum_possible_points":"u128","maximum_number_gold_coins":"u16","maximum_number_silver_coins":"u16"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            gold_coins: number;
            silver_coins: number;
            time: number | string | bigint;
            points: number | string | bigint;
            maximum_possible_points: number | string | bigint;
            maximum_number_gold_coins: number;
            maximum_number_silver_coins: number;
          },
        );
      }
    });
  }

  public subscribeToGameStartedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'GameStarted') {
        callback(null);
      }
    });
  }

  public subscribeToAdminAddedEvent(callback: (data: ActorId) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'AdminAdded') {
        callback(
          this._program.registry
            .createType('(String, String, [u8;32])', message.payload)[2]
            .toJSON() as unknown as ActorId,
        );
      }
    });
  }

  public subscribeToStatusChangedEvent(callback: (data: Status) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'StatusChanged') {
        callback(
          this._program.registry
            .createType('(String, String, Status)', message.payload)[2]
            .toJSON() as unknown as Status,
        );
      }
    });
  }

  public subscribeToConfigChangedEvent(callback: (data: Config) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'ConfigChanged') {
        callback(
          this._program.registry
            .createType('(String, String, Config)', message.payload)[2]
            .toJSON() as unknown as Config,
        );
      }
    });
  }

  public subscribeToLeftGameEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'LeftGame') {
        callback(null);
      }
    });
  }

  public subscribeToKilledEvent(callback: (data: { inheritor: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'VaraMan' && getFnNamePrefix(payload) === 'Killed') {
        callback(
          this._program.registry
            .createType('(String, String, {"inheritor":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { inheritor: ActorId },
        );
      }
    });
  }
}
