import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

type ActorId = string;
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

export class Program {
  public readonly registry: TypeRegistry;
  public readonly session: Session;
  public readonly ticTacToe: TicTacToe;

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      Config: {
        s_per_block: 'u64',
        gas_to_remove_game: 'u64',
        gas_to_delete_session: 'u64',
        time_interval: 'u32',
        turn_deadline_ms: 'u64',
        minimum_session_duration_ms: 'u64',
      },
      SignatureData: { key: '[u8;32]', duration: 'u64', allowed_actions: 'Vec<ActionsForSession>' },
      ActionsForSession: { _enum: ['StartGame', 'Move', 'Skip'] },
      SessionData: {
        key: '[u8;32]',
        expires: 'u64',
        allowed_actions: 'Vec<ActionsForSession>',
        expires_at_block: 'u32',
      },
      GameInstance: {
        board: 'Vec<Option<Mark>>',
        player_mark: 'Mark',
        bot_mark: 'Mark',
        last_time: 'u64',
        game_over: 'bool',
        game_result: 'Option<GameResult>',
      },
      Mark: { _enum: ['X', 'O'] },
      GameResult: { _enum: ['Player', 'Bot', 'Draw'] },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.session = new Session(this);
    this.ticTacToe = new TicTacToe(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, config: Config): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', config],
      '(String, Config)',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, config: Config) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', config],
      '(String, Config)',
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

export class TicTacToe {
  constructor(private _program: Program) {}

  public addAdmin(admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['TicTacToe', 'AddAdmin', admin],
      '(String, String, [u8;32])',
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
      ['TicTacToe', 'AllowMessages', messages_allowed],
      '(String, String, bool)',
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
      ['TicTacToe', 'RemoveAdmin', admin],
      '(String, String, [u8;32])',
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
      ['TicTacToe', 'RemoveGameInstance', account],
      '(String, String, [u8;32])',
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
      ['TicTacToe', 'RemoveGameInstances', accounts],
      '(String, String, Option<Vec<[u8;32]>>)',
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
      ['TicTacToe', 'Skip', session_for_account],
      '(String, String, Option<[u8;32]>)',
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
      ['TicTacToe', 'StartGame', session_for_account],
      '(String, String, Option<[u8;32]>)',
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
      ['TicTacToe', 'Turn', step, session_for_account],
      '(String, String, u8, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public updateConfig(
    s_per_block: number | string | bigint | null,
    gas_to_remove_game: number | string | bigint | null,
    time_interval: number | null,
    turn_deadline_ms: number | string | bigint | null,
    gas_to_delete_session: number | string | bigint | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      [
        'TicTacToe',
        'UpdateConfig',
        s_per_block,
        gas_to_remove_game,
        time_interval,
        turn_deadline_ms,
        gas_to_delete_session,
      ],
      '(String, String, Option<u64>, Option<u64>, Option<u32>, Option<u64>, Option<u64>)',
      'Null',
      this._program.programId,
    );
  }

  public async admins(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<ActorId>> {
    const payload = this._program.registry.createType('(String, String)', ['TicTacToe', 'Admins']).toHex();
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

  public async allGames(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, GameInstance]>> {
    const payload = this._program.registry.createType('(String, String)', ['TicTacToe', 'AllGames']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], GameInstance)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, GameInstance]>;
  }

  public async config(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Config> {
    const payload = this._program.registry.createType('(String, String)', ['TicTacToe', 'Config']).toHex();
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

  public async game(
    player_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<GameInstance | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['TicTacToe', 'Game', player_id])
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
    const result = this._program.registry.createType('(String, String, Option<GameInstance>)', reply.payload);
    return result[2].toJSON() as unknown as GameInstance | null;
  }

  public async messagesAllowed(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<boolean> {
    const payload = this._program.registry.createType('(String, String)', ['TicTacToe', 'MessagesAllowed']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, bool)', reply.payload);
    return result[2].toJSON() as unknown as boolean;
  }

  public subscribeToGameFinishedEvent(
    callback: (data: { game: GameInstance; player_address: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'GameFinished') {
        callback(
          this._program.registry
            .createType('(String, String, {"game":"GameInstance","player_address":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { game: GameInstance; player_address: ActorId },
        );
      }
    });
  }

  public subscribeToGameStartedEvent(
    callback: (data: { game: GameInstance }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'GameStarted') {
        callback(
          this._program.registry
            .createType('(String, String, {"game":"GameInstance"})', message.payload)[2]
            .toJSON() as unknown as { game: GameInstance },
        );
      }
    });
  }

  public subscribeToMoveMadeEvent(
    callback: (data: { game: GameInstance }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'TicTacToe' && getFnNamePrefix(payload) === 'MoveMade') {
        callback(
          this._program.registry
            .createType('(String, String, {"game":"GameInstance"})', message.payload)[2]
            .toJSON() as unknown as { game: GameInstance },
        );
      }
    });
  }

  public subscribeToGameInstanceRemovedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
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
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
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
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
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
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
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
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
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
