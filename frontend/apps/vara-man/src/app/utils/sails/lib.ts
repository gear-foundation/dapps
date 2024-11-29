import { TransactionBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';
import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';

type ActorId = string;

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
  | 'createNewTournament'
  | 'registerForTournament'
  | 'cancelRegister'
  | 'cancelTournament'
  | 'deletePlayer'
  | 'finishSingleGame'
  | 'startTournament'
  | 'recordTournamentResult'
  | 'leaveGame';

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

export class Program {
  public readonly registry: TypeRegistry;
  public readonly session: Session;
  public readonly varaMan: VaraMan;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
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

    this.session = new Session(this);
    this.varaMan = new VaraMan(this);
  }

  newCtorFromCode(
    code: Uint8Array | Buffer,
    config: Config,
    dns_id_and_name: [ActorId, string] | null,
  ): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', config, dns_id_and_name],
      '(String, Config, Option<([u8;32], String)>)',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, config: Config, dns_id_and_name: [ActorId, string] | null) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', config, dns_id_and_name],
      '(String, Config, Option<([u8;32], String)>)',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
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
      ['Session', 'CreateSession', signature_data, signature],
      '(String, String, SignatureData, Option<Vec<u8>>)',
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
      ['Session', 'DeleteSessionFromAccount'],
      '(String, String)',
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
      ['Session', 'DeleteSessionFromProgram', session_for_account],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public async sessionForTheAccount(
    account: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<SessionData | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Session', 'SessionForTheAccount', account])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Option<SessionData>)', reply.payload);
    return result[2].toJSON() as unknown as SessionData | null;
  }

  public async sessions(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, SessionData]>> {
    const payload = this._program.registry.createType('(String, String)', ['Session', 'Sessions']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], SessionData)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, SessionData]>;
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
  constructor(private _program: Program) {}

  public addAdmin(new_admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['VaraMan', 'AddAdmin', new_admin_id],
      '(String, String, [u8;32])',
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
      ['VaraMan', 'CancelRegister', session_for_account],
      '(String, String, Option<[u8;32]>)',
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
      ['VaraMan', 'CancelTournament', session_for_account],
      '(String, String, Option<[u8;32]>)',
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
      ['VaraMan', 'ChangeConfig', config],
      '(String, String, Config)',
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
      ['VaraMan', 'ChangeStatus', status],
      '(String, String, Status)',
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
      ['VaraMan', 'CreateNewTournament', tournament_name, name, level, duration_ms, session_for_account],
      '(String, String, String, String, Level, u32, Option<[u8;32]>)',
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
      ['VaraMan', 'DeletePlayer', player_id, session_for_account],
      '(String, String, [u8;32], Option<[u8;32]>)',
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
      ['VaraMan', 'FinishSingleGame', gold_coins, silver_coins, level, session_for_account],
      '(String, String, u16, u16, Level, Option<[u8;32]>)',
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
      ['VaraMan', 'FinishTournament', admin_id, time_start],
      '(String, String, [u8;32], u64)',
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
      ['VaraMan', 'Kill', inheritor],
      '(String, String, [u8;32])',
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
      ['VaraMan', 'LeaveGame', session_for_account],
      '(String, String, Option<[u8;32]>)',
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
      ['VaraMan', 'RecordTournamentResult', time, gold_coins, silver_coins, session_for_account],
      '(String, String, u128, u16, u16, Option<[u8;32]>)',
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
      ['VaraMan', 'RegisterForTournament', admin_id, name, session_for_account],
      '(String, String, [u8;32], String, Option<[u8;32]>)',
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
      ['VaraMan', 'StartTournament', session_for_account],
      '(String, String, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public async admins(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<ActorId>> {
    const payload = this._program.registry.createType('(String, String)', ['VaraMan', 'Admins']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Vec<[u8;32]>)', reply.payload);
    return result[2].toJSON() as unknown as Array<ActorId>;
  }

  public async all(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<VaraManState> {
    const payload = this._program.registry.createType('(String, String)', ['VaraMan', 'All']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, VaraManState)', reply.payload);
    return result[2].toJSON() as unknown as VaraManState;
  }

  public async config(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Config> {
    const payload = this._program.registry.createType('(String, String)', ['VaraMan', 'Config']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Config)', reply.payload);
    return result[2].toJSON() as unknown as Config;
  }

  public async getTournament(
    player_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<[TournamentState, number | string | bigint | null] | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['VaraMan', 'GetTournament', player_id])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType(
      '(String, String, Option<(TournamentState, Option<u64>)>)',
      reply.payload,
    );
    return result[2].toJSON() as unknown as [TournamentState, number | string | bigint | null] | null;
  }

  public async status(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Status> {
    const payload = this._program.registry.createType('(String, String)', ['VaraMan', 'Status']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Status)', reply.payload);
    return result[2].toJSON() as unknown as Status;
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
