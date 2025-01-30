import { TransactionBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';
import { GearApi, HexString, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';

type ActorId = HexString;

export interface Config {
  reservation_amount: number | string | bigint;
  reservation_duration_in_block: number;
  time_for_step: number;
  min_gas_limit: number | string | bigint;
  gas_refill_timeout: number;
  gas_for_step: number | string | bigint;
}

export interface GameState {
  admin_id: ActorId;
  properties_in_bank: `0x${string}`;
  round: number | string | bigint;
  players: Array<[ActorId, PlayerInfoState]>;
  owners_to_strategy_ids: Array<[ActorId, ActorId]>;
  players_queue: Array<ActorId>;
  current_turn: number;
  current_player: ActorId;
  current_step: number | string | bigint;
  properties: Array<[ActorId, Array<Gear>, number, number] | null>;
  ownership: Array<ActorId>;
  game_status: GameStatus;
  winner: ActorId;
  reservations: Array<ReservationId>;
  entry_fee: number | string | bigint | null;
  prize_pool: number | string | bigint;
}

export interface PlayerInfoState {
  owner_id: ActorId;
  name: string;
  position: number;
  balance: number;
  debt: number;
  in_jail: boolean;
  round: number | string | bigint;
  cells: `0x${string}`;
  penalty: number;
  lost: boolean;
  reservation_id: ReservationId | null;
}

export type ReservationId = [Array<number>];

export type Gear = 'Bronze' | 'Silver' | 'Gold';

export type GameStatus =
  | { registration: null }
  | { play: null }
  | { finished: null }
  | { wait: null }
  | { waitingForGasForGameContract: null }
  | { waitingForGasForStrategy: ActorId };

export class Program {
  public readonly registry: TypeRegistry;
  public readonly syndote: Syndote;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
    const types: Record<string, any> = {
      Config: {
        reservation_amount: 'u64',
        reservation_duration_in_block: 'u32',
        time_for_step: 'u32',
        min_gas_limit: 'u64',
        gas_refill_timeout: 'u32',
        gas_for_step: 'u64',
      },
      GameState: {
        admin_id: '[u8;32]',
        properties_in_bank: 'Vec<u8>',
        round: 'u128',
        players: 'Vec<([u8;32], PlayerInfoState)>',
        owners_to_strategy_ids: 'Vec<([u8;32], [u8;32])>',
        players_queue: 'Vec<[u8;32]>',
        current_turn: 'u8',
        current_player: '[u8;32]',
        current_step: 'u64',
        properties: 'Vec<Option<([u8;32], Vec<Gear>, u32, u32)>>',
        ownership: 'Vec<[u8;32]>',
        game_status: 'GameStatus',
        winner: '[u8;32]',
        reservations: 'Vec<ReservationId>',
        entry_fee: 'Option<u128>',
        prize_pool: 'u128',
      },
      PlayerInfoState: {
        owner_id: '[u8;32]',
        name: 'String',
        position: 'u8',
        balance: 'u32',
        debt: 'u32',
        in_jail: 'bool',
        round: 'u128',
        cells: 'Vec<u8>',
        penalty: 'u8',
        lost: 'bool',
        reservation_id: 'Option<ReservationId>',
      },
      ReservationId: '([u8; 32])',
      Gear: { _enum: ['Bronze', 'Silver', 'Gold'] },
      GameStatus: {
        _enum: {
          Registration: 'Null',
          Play: 'Null',
          Finished: 'Null',
          Wait: 'Null',
          WaitingForGasForGameContract: 'Null',
          WaitingForGasForStrategy: '[u8;32]',
        },
      },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.syndote = new Syndote(this);
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

export class Syndote {
  constructor(private _program: Program) {}

  public addGasToPlayerStrategy(admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'AddGasToPlayerStrategy', admin_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public addGear(admin_id: ActorId, properties_for_sale: `0x${string}` | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'AddGear', admin_id, properties_for_sale],
      '(String, String, [u8;32], Option<Vec<u8>>)',
      'Null',
      this._program.programId,
    );
  }

  public buyCell(admin_id: ActorId, properties_for_sale: `0x${string}` | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'BuyCell', admin_id, properties_for_sale],
      '(String, String, [u8;32], Option<Vec<u8>>)',
      'Null',
      this._program.programId,
    );
  }

  public cancelGameSession(admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'CancelGameSession', admin_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public changeAdmin(admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'ChangeAdmin', admin],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public createGameSession(
    entry_fee: number | string | bigint | null,
    name: string,
    strategy_id: ActorId,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'CreateGameSession', entry_fee, name, strategy_id],
      '(String, String, Option<u128>, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public deleteGame(admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'DeleteGame', admin_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public deletePlayer(player_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'DeletePlayer', player_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public exitGame(admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'ExitGame', admin_id],
      '(String, String, [u8;32])',
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
      ['Syndote', 'Kill', inheritor],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public makeReservation(admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'MakeReservation', admin_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public payRent(admin_id: ActorId, properties_for_sale: `0x${string}` | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'PayRent', admin_id, properties_for_sale],
      '(String, String, [u8;32], Option<Vec<u8>>)',
      'Null',
      this._program.programId,
    );
  }

  public play(admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'Play', admin_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public register(admin_id: ActorId, strategy_id: ActorId, name: string): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'Register', admin_id, strategy_id, name],
      '(String, String, [u8;32], [u8;32], String)',
      'Null',
      this._program.programId,
    );
  }

  public skip(admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'Skip', admin_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public throwRoll(
    admin_id: ActorId,
    pay_fine: boolean,
    properties_for_sale: `0x${string}` | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'ThrowRoll', admin_id, pay_fine, properties_for_sale],
      '(String, String, [u8;32], bool, Option<Vec<u8>>)',
      'Null',
      this._program.programId,
    );
  }

  public upgrade(admin_id: ActorId, properties_for_sale: `0x${string}` | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Syndote', 'Upgrade', admin_id, properties_for_sale],
      '(String, String, [u8;32], Option<Vec<u8>>)',
      'Null',
      this._program.programId,
    );
  }

  public async getConfig(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Config> {
    const payload = this._program.registry.createType('(String, String)', ['Syndote', 'GetConfig']).toHex();
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

  public async getGameSession(
    account_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<GameState | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Syndote', 'GetGameSession', account_id])
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
    const result = this._program.registry.createType('(String, String, Option<GameState>)', reply.payload);
    return result[2].toJSON() as unknown as GameState | null;
  }

  public async getOwnerId(
    admin_id: ActorId,
    strategy_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<ActorId | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32], [u8;32])', ['Syndote', 'GetOwnerId', admin_id, strategy_id])
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
    const result = this._program.registry.createType('(String, String, Option<[u8;32]>)', reply.payload);
    return result[2].toJSON() as unknown as ActorId | null;
  }

  public async getPlayerInfo(
    account_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<PlayerInfoState | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Syndote', 'GetPlayerInfo', account_id])
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
    const result = this._program.registry.createType('(String, String, Option<PlayerInfoState>)', reply.payload);
    return result[2].toJSON() as unknown as PlayerInfoState | null;
  }

  public async getPlayersToSessions(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, ActorId]>> {
    const payload = this._program.registry.createType('(String, String)', ['Syndote', 'GetPlayersToSessions']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], [u8;32])>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, ActorId]>;
  }

  public subscribeToGameSessionCreatedEvent(
    callback: (data: { admin_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'GameSessionCreated') {
        callback(
          this._program.registry
            .createType('(String, String, {"admin_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { admin_id: ActorId },
        );
      }
    });
  }

  public subscribeToReservationMadeEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'ReservationMade') {
        callback(null);
      }
    });
  }

  public subscribeToStrategyRegisteredEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'StrategyRegistered') {
        callback(null);
      }
    });
  }

  public subscribeToGameFinishedEvent(
    callback: (data: { admin_id: ActorId; winner: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'GameFinished') {
        callback(
          this._program.registry
            .createType('(String, String, {"admin_id":"[u8;32]","winner":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { admin_id: ActorId; winner: ActorId },
        );
      }
    });
  }

  public subscribeToGasForPlayerStrategyAddedEvent(
    callback: (data: null) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'GasForPlayerStrategyAdded') {
        callback(null);
      }
    });
  }

  public subscribeToGameWasCancelledEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'GameWasCancelled') {
        callback(null);
      }
    });
  }

  public subscribeToPlayerLeftGameEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'PlayerLeftGame') {
        callback(null);
      }
    });
  }

  public subscribeToStepEvent(
    callback: (data: {
      players: Array<[ActorId, PlayerInfoState]>;
      properties: Array<[ActorId, Array<Gear>, number, number] | null>;
      current_player: ActorId;
      ownership: Array<ActorId>;
      current_step: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'Step') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"players":"Vec<([u8;32], PlayerInfoState)>","properties":"Vec<Option<([u8;32], Vec<Gear>, u32, u32)>>","current_player":"[u8;32]","ownership":"Vec<[u8;32]>","current_step":"u64"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            players: Array<[ActorId, PlayerInfoState]>;
            properties: Array<[ActorId, Array<Gear>, number, number] | null>;
            current_player: ActorId;
            ownership: Array<ActorId>;
            current_step: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToNextRoundFromReservationEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'NextRoundFromReservation') {
        callback(null);
      }
    });
  }

  public subscribeToGameDeletedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'GameDeleted') {
        callback(null);
      }
    });
  }

  public subscribeToPlayerDeletedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'PlayerDeleted') {
        callback(null);
      }
    });
  }

  public subscribeToStrategicSuccessEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'StrategicSuccess') {
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
      if (getServiceNamePrefix(payload) === 'Syndote' && getFnNamePrefix(payload) === 'Killed') {
        callback(
          this._program.registry
            .createType('(String, String, {"inheritor":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { inheritor: ActorId },
        );
      }
    });
  }
}
