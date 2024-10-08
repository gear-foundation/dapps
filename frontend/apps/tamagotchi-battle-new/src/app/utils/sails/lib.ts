import { ActorId, TransactionBuilder, CodeId, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';
import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';

export interface Config {
  health: number;
  max_participants: number;
  attack_range: [number, number];
  defence_range: [number, number];
  dodge_range: [number, number];
  available_points: number;
  time_for_move_in_blocks: number;
  block_duration_ms: number;
  gas_to_check_pair: number | string | bigint;
  gas_for_create_warrior: number | string | bigint;
  gas_to_cancel_the_battle: number | string | bigint;
  time_to_cancel_the_battle: number;
  reservation_amount: number | string | bigint;
  reservation_time: number;
}

export type Move = 'attack' | 'reflect' | 'ultimate';

export interface BattleState {
  admin: ActorId;
  time_creation: number | string | bigint;
  bid: number | string | bigint;
  participants: Array<[ActorId, Player]>;
  defeated_participants: Array<[ActorId, Player]>;
  state: State;
  pairs: Array<[number, Pair]>;
  players_to_pairs: Array<[ActorId, number]>;
  waiting_player: [ActorId, number] | null;
  pair_id: number;
  reservation: Array<[ActorId, ReservationId]>;
}

export interface Player {
  warrior_id: ActorId;
  owner: ActorId;
  name: string;
  player_settings: PlayerSettings;
  appearance_identifiers: `0x${string}`;
  number_of_victories: number;
  ultimate_reload: number;
  reflect_reload: number;
}

export interface PlayerSettings {
  health: number;
  attack: number;
  defence: number;
  dodge: number;
}

export type State = { registration: null } | { started: null } | { gameIsOver: { winner: ActorId } };

export interface Pair {
  player_1: ActorId;
  player_2: ActorId;
  action: [ActorId, Move] | null;
  round: number;
  round_start_time: number | string | bigint;
}

export type ReservationId = [Array<number>];

export class Program {
  public readonly registry: TypeRegistry;
  public readonly battle: Battle;

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      Config: {
        health: 'u16',
        max_participants: 'u8',
        attack_range: '(u16, u16)',
        defence_range: '(u16, u16)',
        dodge_range: '(u16, u16)',
        available_points: 'u16',
        time_for_move_in_blocks: 'u32',
        block_duration_ms: 'u32',
        gas_to_check_pair: 'u64',
        gas_for_create_warrior: 'u64',
        gas_to_cancel_the_battle: 'u64',
        time_to_cancel_the_battle: 'u32',
        reservation_amount: 'u64',
        reservation_time: 'u32',
      },
      Move: { _enum: ['Attack', 'Reflect', 'Ultimate'] },
      BattleState: {
        admin: '[u8;32]',
        time_creation: 'u64',
        bid: 'u128',
        participants: 'Vec<([u8;32], Player)>',
        defeated_participants: 'Vec<([u8;32], Player)>',
        state: 'State',
        pairs: 'Vec<(u16, Pair)>',
        players_to_pairs: 'Vec<([u8;32], u16)>',
        waiting_player: 'Option<([u8;32], u16)>',
        pair_id: 'u16',
        reservation: 'Vec<([u8;32], ReservationId)>',
      },
      Player: {
        warrior_id: '[u8;32]',
        owner: '[u8;32]',
        name: 'String',
        player_settings: 'PlayerSettings',
        appearance_identifiers: 'Vec<u8>',
        number_of_victories: 'u8',
        ultimate_reload: 'u8',
        reflect_reload: 'u8',
      },
      PlayerSettings: { health: 'u16', attack: 'u16', defence: 'u16', dodge: 'u16' },
      State: { _enum: { Registration: 'Null', Started: 'Null', GameIsOver: { winner: '[u8;32]' } } },
      Pair: {
        player_1: '[u8;32]',
        player_2: '[u8;32]',
        action: 'Option<([u8;32], Move)>',
        round: 'u8',
        round_start_time: 'u64',
      },
      ReservationId: '([u8; 32])',
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.battle = new Battle(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, config: Config, warrior_code_id: CodeId): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', config, warrior_code_id],
      '(String, Config, [u8;32])',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, config: Config, warrior_code_id: CodeId) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', config, warrior_code_id],
      '(String, Config, [u8;32])',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class Battle {
  constructor(private _program: Program) {}

  public addAdmin(new_admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'AddAdmin', new_admin],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public automaticMove(player_id: ActorId, number_of_victories: number, round: number): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'AutomaticMove', player_id, number_of_victories, round],
      '(String, String, [u8;32], u8, u8)',
      'Null',
      this._program.programId,
    );
  }

  public cancelRegister(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'CancelRegister'],
      '(String, String)',
      'Null',
      this._program.programId,
    );
  }

  public cancelTournament(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'CancelTournament'],
      '(String, String)',
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
      ['Battle', 'ChangeConfig', config],
      '(String, String, Config)',
      'Null',
      this._program.programId,
    );
  }

  public changeWarriorCodeId(warrior_code_id: CodeId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'ChangeWarriorCodeId', warrior_code_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public createNewBattle(
    name: string,
    warrior_id: ActorId,
    attack: number,
    defence: number,
    dodge: number,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'CreateNewBattle', name, warrior_id, attack, defence, dodge],
      '(String, String, String, [u8;32], u16, u16, u16)',
      'Null',
      this._program.programId,
    );
  }

  public delayedCancelTournament(game_id: ActorId, time_creation: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'DelayedCancelTournament', game_id, time_creation],
      '(String, String, [u8;32], u64)',
      'Null',
      this._program.programId,
    );
  }

  public generateWarrior(appearance_identifiers: `0x${string}`): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'GenerateWarrior', appearance_identifiers],
      '(String, String, Vec<u8>)',
      'Null',
      this._program.programId,
    );
  }

  public makeMove(warrior_move: Move): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'MakeMove', warrior_move],
      '(String, String, Move)',
      'Null',
      this._program.programId,
    );
  }

  public register(
    game_id: ActorId,
    warrior_id: ActorId,
    name: string,
    attack: number,
    defence: number,
    dodge: number,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'Register', game_id, warrior_id, name, attack, defence, dodge],
      '(String, String, [u8;32], [u8;32], String, u16, u16, u16)',
      'Null',
      this._program.programId,
    );
  }

  public startBattle(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'StartBattle'],
      '(String, String)',
      'Null',
      this._program.programId,
    );
  }

  public startNextFight(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'StartNextFight'],
      '(String, String)',
      'Null',
      this._program.programId,
    );
  }

  public async admins(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<ActorId>> {
    const payload = this._program.registry.createType('(String, String)', ['Battle', 'Admins']).toHex();
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

  public async config(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Config> {
    const payload = this._program.registry.createType('(String, String)', ['Battle', 'Config']).toHex();
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

  public async getBattle(
    game_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<BattleState | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Battle', 'GetBattle', game_id])
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
    const result = this._program.registry.createType('(String, String, Option<BattleState>)', reply.payload);
    return result[2].toJSON() as unknown as BattleState | null;
  }

  public async getMyBattle(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<BattleState | null> {
    const payload = this._program.registry.createType('(String, String)', ['Battle', 'GetMyBattle']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Option<BattleState>)', reply.payload);
    return result[2].toJSON() as unknown as BattleState | null;
  }

  public async warriorCodeId(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<CodeId> {
    const payload = this._program.registry.createType('(String, String)', ['Battle', 'WarriorCodeId']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, [u8;32])', reply.payload);
    return result[2].toJSON() as unknown as CodeId;
  }

  public subscribeToNewBattleCreatedEvent(
    callback: (data: { battle_id: ActorId; bid: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'NewBattleCreated') {
        callback(
          this._program.registry
            .createType('(String, String, {"battle_id":"[u8;32]","bid":"u128"})', message.payload)[2]
            .toJSON() as unknown as { battle_id: ActorId; bid: number | string | bigint },
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
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'PlayerRegistered') {
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
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'RegisterCanceled') {
        callback(null);
      }
    });
  }

  public subscribeToBattleCanceledEvent(
    callback: (data: { game_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'BattleCanceled') {
        callback(
          this._program.registry
            .createType('(String, String, {"game_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { game_id: ActorId },
        );
      }
    });
  }

  public subscribeToBattleStartedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'BattleStarted') {
        callback(null);
      }
    });
  }

  public subscribeToMoveMadeEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'MoveMade') {
        callback(null);
      }
    });
  }

  public subscribeToBattleFinishedEvent(
    callback: (data: { winner: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'BattleFinished') {
        callback(
          this._program.registry
            .createType('(String, String, {"winner":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { winner: ActorId },
        );
      }
    });
  }

  public subscribeToPairCheckedEvent(
    callback: (data: { game_id: ActorId; pair_id: number; round: number }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'PairChecked') {
        callback(
          this._program.registry
            .createType('(String, String, {"game_id":"[u8;32]","pair_id":"u8","round":"u8"})', message.payload)[2]
            .toJSON() as unknown as { game_id: ActorId; pair_id: number; round: number },
        );
      }
    });
  }

  public subscribeToFirstRoundCheckedEvent(
    callback: (data: { game_id: ActorId; wave: number }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'FirstRoundChecked') {
        callback(
          this._program.registry
            .createType('(String, String, {"game_id":"[u8;32]","wave":"u8"})', message.payload)[2]
            .toJSON() as unknown as { game_id: ActorId; wave: number },
        );
      }
    });
  }

  public subscribeToNextBattleStartedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'NextBattleStarted') {
        callback(null);
      }
    });
  }

  public subscribeToEnemyWaitingEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'EnemyWaiting') {
        callback(null);
      }
    });
  }

  public subscribeToWarriorGeneratedEvent(
    callback: (data: { address: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'WarriorGenerated') {
        callback(
          this._program.registry
            .createType('(String, String, {"address":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { address: ActorId },
        );
      }
    });
  }

  public subscribeToAdminAddedEvent(
    callback: (data: { new_admin: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'AdminAdded') {
        callback(
          this._program.registry
            .createType('(String, String, {"new_admin":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { new_admin: ActorId },
        );
      }
    });
  }

  public subscribeToConfigChangedEvent(
    callback: (data: { config: Config }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'ConfigChanged') {
        callback(
          this._program.registry
            .createType('(String, String, {"config":"Config"})', message.payload)[2]
            .toJSON() as unknown as { config: Config },
        );
      }
    });
  }

  public subscribeToWarriorCodeIdChangedEvent(
    callback: (data: { warrior_code_id: CodeId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'WarriorCodeIdChanged') {
        callback(
          this._program.registry
            .createType('(String, String, {"warrior_code_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { warrior_code_id: CodeId },
        );
      }
    });
  }
}
