import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

export type ActorId = string;

export interface UtilsConfig {
  health: number;
  max_participants: number;
  attack_range: [number, number];
  defence_range: [number, number];
  dodge_range: [number, number];
  available_points: number;
  time_for_move_in_blocks: number;
  block_duration_ms: number;
  gas_for_create_warrior: number | string | bigint;
  gas_to_cancel_the_battle: number | string | bigint;
  time_to_cancel_the_battle: number;
  reservation_amount: number | string | bigint;
  reservation_time: number;
}

export interface SessionConfig {
  gas_to_delete_session: number | string | bigint;
  minimum_session_duration_ms: number | string | bigint;
  ms_per_block: number | string | bigint;
}

export interface Appearance {
  head_index: number;
  hat_index: number;
  body_index: number;
  accessory_index: number;
  body_color: string;
  back_color: string;
}

export type Move = 'Attack' | 'Reflect' | 'Ultimate';

export interface BattleState {
  admin: ActorId;
  battle_name: string;
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
  warrior_id: ActorId | null;
  owner: ActorId;
  user_name: string;
  player_settings: PlayerSettings;
  appearance: Appearance;
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

export type State = { registration: null } | { started: null } | { gameIsOver: { winners: [ActorId, ActorId | null] } };

export interface Pair {
  player_1: ActorId;
  player_2: ActorId;
  action: [ActorId, Move] | null;
  round: number;
  round_start_time: number | string | bigint;
}

/**
 * Reservation identifier.
 *
 * The identifier is used to reserve and unreserve gas amount for program
 * execution later.
 */
export type ReservationId = [Array<number>];

export interface SignatureData {
  key: ActorId;
  duration: number | string | bigint;
  allowed_actions: Array<ActionsForSession>;
}

export type ActionsForSession = 'CreateNewBattle' | 'Registration' | 'StartBattle' | 'MakeMove';

export interface SessionData {
  key: ActorId;
  expires: number | string | bigint;
  allowed_actions: Array<ActionsForSession>;
  expires_at_block: number;
}

export class Program {
  public readonly registry: TypeRegistry;
  public readonly battle: Battle;
  public readonly session: Session;

  constructor(
    public api: GearApi,
    private _programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      UtilsConfig: {
        health: 'u16',
        max_participants: 'u8',
        attack_range: '(u16, u16)',
        defence_range: '(u16, u16)',
        dodge_range: '(u16, u16)',
        available_points: 'u16',
        time_for_move_in_blocks: 'u32',
        block_duration_ms: 'u32',
        gas_for_create_warrior: 'u64',
        gas_to_cancel_the_battle: 'u64',
        time_to_cancel_the_battle: 'u32',
        reservation_amount: 'u64',
        reservation_time: 'u32',
      },
      SessionConfig: { gas_to_delete_session: 'u64', minimum_session_duration_ms: 'u64', ms_per_block: 'u64' },
      Appearance: {
        head_index: 'u16',
        hat_index: 'u16',
        body_index: 'u16',
        accessory_index: 'u16',
        body_color: 'String',
        back_color: 'String',
      },
      Move: { _enum: ['Attack', 'Reflect', 'Ultimate'] },
      BattleState: {
        admin: '[u8;32]',
        battle_name: 'String',
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
        warrior_id: 'Option<[u8;32]>',
        owner: '[u8;32]',
        user_name: 'String',
        player_settings: 'PlayerSettings',
        appearance: 'Appearance',
        number_of_victories: 'u8',
        ultimate_reload: 'u8',
        reflect_reload: 'u8',
      },
      PlayerSettings: { health: 'u16', attack: 'u16', defence: 'u16', dodge: 'u16' },
      State: {
        _enum: { Registration: 'Null', Started: 'Null', GameIsOver: { winners: '([u8;32], Option<[u8;32]>)' } },
      },
      Pair: {
        player_1: '[u8;32]',
        player_2: '[u8;32]',
        action: 'Option<([u8;32], Move)>',
        round: 'u8',
        round_start_time: 'u64',
      },
      ReservationId: '([u8; 32])',
      SignatureData: { key: '[u8;32]', duration: 'u64', allowed_actions: 'Vec<ActionsForSession>' },
      ActionsForSession: { _enum: ['CreateNewBattle', 'Registration', 'StartBattle', 'MakeMove'] },
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

    this.battle = new Battle(this);
    this.session = new Session(this);
  }

  public get programId(): `0x${string}` {
    if (!this._programId) throw new Error(`Program ID is not set`);
    return this._programId;
  }

  newCtorFromCode(
    code: ArrayBuffer | Uint8Array,
    config: UtilsConfig,
    session_config: SessionConfig,
  ): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', config, session_config],
      '(String, UtilsConfig, SessionConfig)',
      'String',
      code,
    );

    this._programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, config: UtilsConfig, session_config: SessionConfig) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', config, session_config],
      '(String, UtilsConfig, SessionConfig)',
      'String',
      codeId,
    );

    this._programId = builder.programId;
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

  public cancelRegister(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'CancelRegister', session_for_account],
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
      ['Battle', 'CancelTournament', session_for_account],
      '(String, String, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public changeConfig(config: UtilsConfig): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'ChangeConfig', config],
      '(String, String, UtilsConfig)',
      'Null',
      this._program.programId,
    );
  }

  public createNewBattle(
    battle_name: string,
    user_name: string,
    warrior_id: ActorId | null,
    appearance: Appearance | null,
    attack: number,
    defence: number,
    dodge: number,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      [
        'Battle',
        'CreateNewBattle',
        battle_name,
        user_name,
        warrior_id,
        appearance,
        attack,
        defence,
        dodge,
        session_for_account,
      ],
      '(String, String, String, String, Option<[u8;32]>, Option<Appearance>, u16, u16, u16, Option<[u8;32]>)',
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

  public deletePlayer(player_id: ActorId, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'DeletePlayer', player_id, session_for_account],
      '(String, String, [u8;32], Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public exitGame(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'ExitGame', session_for_account],
      '(String, String, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public makeMove(warrior_move: Move, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'MakeMove', warrior_move, session_for_account],
      '(String, String, Move, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public register(
    game_id: ActorId,
    warrior_id: ActorId | null,
    appearance: Appearance | null,
    user_name: string,
    attack: number,
    defence: number,
    dodge: number,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'Register', game_id, warrior_id, appearance, user_name, attack, defence, dodge, session_for_account],
      '(String, String, [u8;32], Option<[u8;32]>, Option<Appearance>, String, u16, u16, u16, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public startBattle(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'StartBattle', session_for_account],
      '(String, String, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public startNextFight(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Battle', 'StartNextFight', session_for_account],
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
    const payload = this._program.registry.createType('(String, String)', ['Battle', 'Admins']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
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
  ): Promise<UtilsConfig> {
    const payload = this._program.registry.createType('(String, String)', ['Battle', 'Config']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, UtilsConfig)', reply.payload);
    return result[2].toJSON() as unknown as UtilsConfig;
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
      destination: this._program.programId,
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
      destination: this._program.programId,
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
    callback: (data: { admin_id: ActorId; user_name: string; bid: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'PlayerRegistered') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"admin_id":"[u8;32]","user_name":"String","bid":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as { admin_id: ActorId; user_name: string; bid: number | string | bigint },
        );
      }
    });
  }

  public subscribeToRegisterCanceledEvent(
    callback: (data: { player_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'RegisterCanceled') {
        callback(
          this._program.registry
            .createType('(String, String, {"player_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { player_id: ActorId },
        );
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
    callback: (data: { config: UtilsConfig }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'ConfigChanged') {
        callback(
          this._program.registry
            .createType('(String, String, {"config":"UtilsConfig"})', message.payload)[2]
            .toJSON() as unknown as { config: UtilsConfig },
        );
      }
    });
  }

  public subscribeToGameLeftEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'GameLeft') {
        callback(null);
      }
    });
  }

  public subscribeToRoundActionEvent(
    callback: (data: {
      round: number;
      player_1: [ActorId, Move, number];
      player_2: [ActorId, Move, number];
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'RoundAction') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"round":"u8","player_1":"([u8;32], Move, u16)","player_2":"([u8;32], Move, u16)"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            round: number;
            player_1: [ActorId, Move, number];
            player_2: [ActorId, Move, number];
          },
        );
      }
    });
  }

  public subscribeToAutomaticMoveMadeEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Battle' && getFnNamePrefix(payload) === 'AutomaticMoveMade') {
        callback(null);
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
      destination: this._program.programId,
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
      destination: this._program.programId,
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
