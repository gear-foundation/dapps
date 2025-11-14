/* eslint-disable */

import { GearApi, BaseGearProgram, HexString } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, ActorId, QueryBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

export class SailsProgram {
  public readonly registry: TypeRegistry;
  public readonly session: Session;
  public readonly ticTacToe: TicTacToe;
  private _program?: BaseGearProgram;

  constructor(public api: GearApi, programId?: `0x${string}`) {
    const types: Record<string, any> = {
      Config: {"s_per_block":"u64","gas_to_remove_game":"u64","gas_to_delete_session":"u64","time_interval":"u32","turn_deadline_ms":"u64","minimum_session_duration_ms":"u64"},
      SignatureData: {"key":"[u8;32]","duration":"u64","allowed_actions":"Vec<ActionsForSession>"},
      ActionsForSession: {"_enum":["StartGame","Move","Skip"]},
      SessionData: {"key":"[u8;32]","expires":"u64","allowed_actions":"Vec<ActionsForSession>","expires_at_block":"u32"},
      GameInstance: {"board":"Vec<Option<Mark>>","player_mark":"Mark","bot_mark":"Mark","last_time":"u64","game_over":"bool","game_result":"Option<GameResult>"},
      Mark: {"_enum":["X","O"]},
      GameResult: {"_enum":["Player","Bot","Draw"]},
    }

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);
    if (programId) {
      this._program = new BaseGearProgram(programId, api);
    }

    this.session = new Session(this);
    this.ticTacToe = new TicTacToe(this);
  }

  public get programId(): `0x${string}` {
    if (!this._program) throw new Error(`Program ID is not set`);
    return this._program.id;
  }

  newCtorFromCode(code: Uint8Array | Buffer | HexString, config: Config): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      config,
      'Config',
      'String',
      code,
      async (programId) =>  {
        this._program = await BaseGearProgram.new(programId, this.api);
      }
    );
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, config: Config) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      null,
      'New',
      config,
      'Config',
      'String',
      codeId,
      async (programId) =>  {
        this._program = await BaseGearProgram.new(programId, this.api);
      }
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
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
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
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
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

export class TicTacToe {
  constructor(private _program: SailsProgram) {}

  public addAdmin(admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'TicTacToe',
      'AddAdmin',
      admin,
      '[u8;32]',
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
      'TicTacToe',
      'AllowMessages',
      messages_allowed,
      'bool',
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
      'TicTacToe',
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
      'TicTacToe',
      'RemoveGameInstance',
      account,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public removeGameInstances(accounts: Array<ActorId> | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'TicTacToe',
      'RemoveGameInstances',
      accounts,
      'Option<Vec<[u8;32]>>',
      'Null',
      this._program.programId,
    );
  }

  public skip(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'TicTacToe',
      'Skip',
      session_for_account,
      'Option<[u8;32]>',
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
      'TicTacToe',
      'StartGame',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  public turn(step: number, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'TicTacToe',
      'Turn',
      [step, session_for_account],
      '(u8, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public updateConfig(s_per_block: number | string | bigint | null, gas_to_remove_game: number | string | bigint | null, time_interval: number | null, turn_deadline_ms: number | string | bigint | null, gas_to_delete_session: number | string | bigint | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'TicTacToe',
      'UpdateConfig',
      [s_per_block, gas_to_remove_game, time_interval, turn_deadline_ms, gas_to_delete_session],
      '(Option<u64>, Option<u64>, Option<u32>, Option<u64>, Option<u64>)',
      'Null',
      this._program.programId,
    );
  }

  public admins(): QueryBuilder<Array<ActorId>> {
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'TicTacToe',
      'Admins',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public allGames(): QueryBuilder<Array<[ActorId, GameInstance]>> {
    return new QueryBuilder<Array<[ActorId, GameInstance]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'TicTacToe',
      'AllGames',
      null,
      null,
      'Vec<([u8;32], GameInstance)>',
    );
  }

  public config(): QueryBuilder<Config> {
    return new QueryBuilder<Config>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'TicTacToe',
      'Config',
      null,
      null,
      'Config',
    );
  }

  public game(player_id: ActorId): QueryBuilder<GameInstance | null> {
    return new QueryBuilder<GameInstance | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'TicTacToe',
      'Game',
      player_id,
      '[u8;32]',
      'Option<GameInstance>',
    );
  }

  public messagesAllowed(): QueryBuilder<boolean> {
    return new QueryBuilder<boolean>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'TicTacToe',
      'MessagesAllowed',
      null,
      null,
      'bool',
    );
  }

  public subscribeToGameFinishedEvent(callback: (data: { game: GameInstance; player_address: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'GameFinished') {
        callback(this._program.registry.createType('(String, String, {"game":"GameInstance","player_address":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { game: GameInstance; player_address: ActorId });
      }
    });
  }

  public subscribeToGameStartedEvent(callback: (data: { game: GameInstance }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'GameStarted') {
        callback(this._program.registry.createType('(String, String, {"game":"GameInstance"})', message.payload)[2].toJSON() as unknown as { game: GameInstance });
      }
    });
  }

  public subscribeToMoveMadeEvent(callback: (data: { game: GameInstance }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'MoveMade') {
        callback(this._program.registry.createType('(String, String, {"game":"GameInstance"})', message.payload)[2].toJSON() as unknown as { game: GameInstance });
      }
    });
  }

  public subscribeToGameInstanceRemovedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'GameInstanceRemoved') {
        callback(null);
      }
    });
  }

  public subscribeToConfigUpdatedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'ConfigUpdated') {
        callback(null);
      }
    });
  }

  public subscribeToAdminRemovedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'AdminRemoved') {
        callback(null);
      }
    });
  }

  public subscribeToAdminAddedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'AdminAdded') {
        callback(null);
      }
    });
  }

  public subscribeToStatusMessagesUpdatedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'StatusMessagesUpdated') {
        callback(null);
      }
    });
  }
}