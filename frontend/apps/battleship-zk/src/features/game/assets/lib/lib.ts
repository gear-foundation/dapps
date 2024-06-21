import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

export type ActorId = string;

export interface VerifyingKeyBytes {
  alpha_g1_beta_g2: Array<number>;
  gamma_g2_neg_pc: Array<number>;
  delta_g2_neg_pc: Array<number>;
  ic: Array<Array<number>>;
}

export interface Configuration {
  gas_for_delete_single_game: number | string;
  gas_for_delete_multiple_game: number | string;
  gas_for_check_time: number | string;
  delay_for_delete_single_game: number;
  delay_for_delete_multiple_game: number;
  delay_for_check_time: number;
}

export interface ProofBytes {
  a: Array<number>;
  b: Array<number>;
  c: Array<number>;
}

export interface PublicMoveInput {
  out: number;
  hit: number;
  hash: Array<number>;
}

export interface PublicStartInput {
  hash: Array<number>;
}

export interface MultipleGameState {
  admin: ActorId;
  participants_data: Array<[ActorId, ParticipantInfo]>;
  create_time: number | string;
  start_time: number | string | null;
  last_move_time: number | string;
  status: MultipleUtilsStatus;
  bid: number | string;
}

export interface ParticipantInfo {
  name: string;
  board: Array<Entity>;
  ship_hash: Array<number>;
  total_shots: number;
  succesfull_shots: number;
}

export type Entity = 'empty' | 'unknown' | 'occupied' | 'ship' | 'boom' | 'boomShip' | 'deadShip';

export type MultipleUtilsStatus = { registration: null } & { verificationPlacement: ActorId | null } & {
  pendingVerificationOfTheMove: [ActorId, number];
} & { turn: ActorId };

export type ActionsForSession = 'playSingleGame' | 'playMultipleGame';

export interface Session {
  key: ActorId;
  expires: number | string;
  allowed_actions: Array<ActionsForSession>;
}

export interface SingleGame {
  player_board: Array<Entity>;
  ship_hash: Array<number>;
  bot_ships: Ships;
  start_time: number | string;
  status: SingleUtilsStatus;
  total_shots: number;
  succesfull_shots: number;
}

export interface Ships {
  ship_1: Array<number>;
  ship_2: Array<number>;
  ship_3: Array<number>;
  ship_4: Array<number>;
}

export type SingleUtilsStatus = { pendingVerificationOfTheMove: number } | { pendingMove: null };

export interface SingleGameState {
  player_board: Array<Entity>;
  ship_hash: Array<number>;
  start_time: number | string;
  status: SingleUtilsStatus;
  total_shots: number;
  succesfull_shots: number;
}

export type BattleshipParticipants = 'Player' | 'Bot';

export type StepResult = 'Missed' | 'Injured' | 'Killed';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly admin: Admin;
  public readonly multiple: Multiple;
  public readonly session: Session;
  public readonly single: Single;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
    const types: Record<string, any> = {
      ActorId: '([u8; 32])',
      VerifyingKeyBytes: {
        alpha_g1_beta_g2: 'Vec<u8>',
        gamma_g2_neg_pc: 'Vec<u8>',
        delta_g2_neg_pc: 'Vec<u8>',
        ic: 'Vec<Vec<u8>>',
      },
      Configuration: {
        gas_for_delete_single_game: 'u64',
        gas_for_delete_multiple_game: 'u64',
        gas_for_check_time: 'u64',
        delay_for_delete_single_game: 'u32',
        delay_for_delete_multiple_game: 'u32',
        delay_for_check_time: 'u32',
      },
      ProofBytes: { a: 'Vec<u8>', b: 'Vec<u8>', c: 'Vec<u8>' },
      PublicMoveInput: { out: 'u8', hit: 'u8', hash: 'Vec<u8>' },
      PublicStartInput: { hash: 'Vec<u8>' },
      MultipleGameState: {
        admin: 'ActorId',
        participants_data: 'Vec<(ActorId, ParticipantInfo)>',
        create_time: 'u64',
        start_time: 'Option<u64>',
        last_move_time: 'u64',
        status: 'MultipleUtilsStatus',
        bid: 'u128',
      },
      ParticipantInfo: {
        name: 'String',
        board: 'Vec<Entity>',
        ship_hash: 'Vec<u8>',
        total_shots: 'u8',
        succesfull_shots: 'u8',
      },
      Entity: { _enum: ['Empty', 'Unknown', 'Occupied', 'Ship', 'Boom', 'BoomShip', 'DeadShip'] },
      MultipleUtilsStatus: {
        _enum: {
          Registration: 'Null',
          VerificationPlacement: 'Option<ActorId>',
          PendingVerificationOfTheMove: '(ActorId, u8)',
          Turn: 'ActorId',
        },
      },
      ActionsForSession: { _enum: ['PlaySingleGame', 'PlayMultipleGame'] },
      Session: { key: 'ActorId', expires: 'u64', allowed_actions: 'Vec<ActionsForSession>' },
      SingleGame: {
        player_board: 'Vec<Entity>',
        ship_hash: 'Vec<u8>',
        bot_ships: 'Ships',
        start_time: 'u64',
        status: 'SingleUtilsStatus',
        total_shots: 'u8',
        succesfull_shots: 'u8',
      },
      Ships: { ship_1: 'Vec<u8>', ship_2: 'Vec<u8>', ship_3: 'Vec<u8>', ship_4: 'Vec<u8>' },
      SingleUtilsStatus: { _enum: { PendingVerificationOfTheMove: 'u8', PendingMove: 'Null' } },
      SingleGameState: {
        player_board: 'Vec<Entity>',
        ship_hash: 'Vec<u8>',
        start_time: 'u64',
        status: 'SingleUtilsStatus',
        total_shots: 'u8',
        succesfull_shots: 'u8',
      },
      BattleshipParticipants: { _enum: ['Player', 'Bot'] },
      StepResult: { _enum: ['Missed', 'Injured', 'Killed'] },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.admin = new Admin(this);
    this.multiple = new Multiple(this);
    this.session = new Session(this);
    this.single = new Single(this);
  }

  newCtorFromCode(
    code: Uint8Array | Buffer,
    builtin_bls381: ActorId,
    verification_key_for_start: VerifyingKeyBytes,
    verification_key_for_move: VerifyingKeyBytes,
    config: Configuration,
  ): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', builtin_bls381, verification_key_for_start, verification_key_for_move, config],
      '(String, ActorId, VerifyingKeyBytes, VerifyingKeyBytes, Configuration)',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(
    codeId: `0x${string}`,
    builtin_bls381: ActorId,
    verification_key_for_start: VerifyingKeyBytes,
    verification_key_for_move: VerifyingKeyBytes,
    config: Configuration,
  ) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', builtin_bls381, verification_key_for_start, verification_key_for_move, config],
      '(String, ActorId, VerifyingKeyBytes, VerifyingKeyBytes, Configuration)',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class Admin {
  constructor(private _program: Program) {}

  public changeAdmin(new_admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Admin', 'ChangeAdmin', new_admin],
      '(String, String, ActorId)',
      'Null',
      this._program.programId,
    );
  }

  public changeBuiltinAddress(new_builtin_address: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Admin', 'ChangeBuiltinAddress', new_builtin_address],
      '(String, String, ActorId)',
      'Null',
      this._program.programId,
    );
  }

  public changeVerificationKey(
    new_vk_for_start: VerifyingKeyBytes | null,
    new_vk_for_move: VerifyingKeyBytes | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Admin', 'ChangeVerificationKey', new_vk_for_start, new_vk_for_move],
      '(String, String, Option<VerifyingKeyBytes>, Option<VerifyingKeyBytes>)',
      'Null',
      this._program.programId,
    );
  }

  public deleteMultipleGame(game_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Admin', 'DeleteMultipleGame', game_id],
      '(String, String, ActorId)',
      'Null',
      this._program.programId,
    );
  }

  public deleteMultipleGamesByTime(time: number | string): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Admin', 'DeleteMultipleGamesByTime', time],
      '(String, String, u64)',
      'Null',
      this._program.programId,
    );
  }

  public deleteMultipleGamesInBatches(divider: number | string): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Admin', 'DeleteMultipleGamesInBatches', divider],
      '(String, String, u64)',
      'Null',
      this._program.programId,
    );
  }

  public deleteSingleGame(player_address: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Admin', 'DeleteSingleGame', player_address],
      '(String, String, ActorId)',
      'Null',
      this._program.programId,
    );
  }

  public deleteSingleGames(time: number | string): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Admin', 'DeleteSingleGames', time],
      '(String, String, u64)',
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
      ['Admin', 'Kill', inheritor],
      '(String, String, ActorId)',
      'Null',
      this._program.programId,
    );
  }

  public async admin(
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<ActorId> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry.createType('(String, String)', '[Admin, Admin]').toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType('(String, String, ActorId)', reply.payload);
    return result[2].toJSON() as unknown as ActorId;
  }

  public async builtin(
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<ActorId> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry.createType('(String, String)', '[Admin, Builtin]').toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType('(String, String, ActorId)', reply.payload);
    return result[2].toJSON() as unknown as ActorId;
  }

  public async verificationKey(
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<[VerifyingKeyBytes, VerifyingKeyBytes]> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry.createType('(String, String)', '[Admin, VerificationKey]').toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType(
      '(String, String, (VerifyingKeyBytes, VerifyingKeyBytes))',
      reply.payload,
    );
    return result[2].toJSON() as unknown as [VerifyingKeyBytes, VerifyingKeyBytes];
  }

  public subscribeToGameDeletedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Admin' && getFnNamePrefix(payload) === 'GameDeleted') {
        callback(null);
      }
    });
  }

  public subscribeToGamesDeletedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Admin' && getFnNamePrefix(payload) === 'GamesDeleted') {
        callback(null);
      }
    });
  }

  public subscribeToAdminChangedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Admin' && getFnNamePrefix(payload) === 'AdminChanged') {
        callback(null);
      }
    });
  }

  public subscribeToBuiltinAddressChangedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Admin' && getFnNamePrefix(payload) === 'BuiltinAddressChanged') {
        callback(null);
      }
    });
  }

  public subscribeToVerificationKeyChangedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Admin' && getFnNamePrefix(payload) === 'VerificationKeyChanged') {
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
      if (getServiceNamePrefix(payload) === 'Admin' && getFnNamePrefix(payload) === 'Killed') {
        callback(
          this._program.registry
            .createType('(String, String, {"inheritor":"ActorId"})', message.payload)[2]
            .toJSON() as { inheritor: ActorId },
        );
      }
    });
  }
}

export class Multiple {
  constructor(private _program: Program) {}

  public cancelGame(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'CancelGame', session_for_account],
      '(String, String, Option<ActorId>)',
      'Null',
      this._program.programId,
    );
  }

  public checkOutTiming(
    game_id: ActorId,
    check_time: number | string,
    repeated_pass: boolean,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'CheckOutTiming', game_id, check_time, repeated_pass],
      '(String, String, ActorId, u64, bool)',
      'Null',
      this._program.programId,
    );
  }

  public createGame(name: string, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'CreateGame', name, session_for_account],
      '(String, String, String, Option<ActorId>)',
      'Null',
      this._program.programId,
    );
  }

  public deleteGame(game_id: ActorId, create_time: number | string): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'DeleteGame', game_id, create_time],
      '(String, String, ActorId, u64)',
      'Null',
      this._program.programId,
    );
  }

  public deletePlayer(removable_player: ActorId, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'DeletePlayer', removable_player, session_for_account],
      '(String, String, ActorId, Option<ActorId>)',
      'Null',
      this._program.programId,
    );
  }

  public joinGame(game_id: ActorId, name: string, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'JoinGame', game_id, name, session_for_account],
      '(String, String, ActorId, String, Option<ActorId>)',
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
      ['Multiple', 'LeaveGame', session_for_account],
      '(String, String, Option<ActorId>)',
      'Null',
      this._program.programId,
    );
  }

  public makeMove(game_id: ActorId, step: number, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'MakeMove', game_id, step, session_for_account],
      '(String, String, ActorId, u8, Option<ActorId>)',
      'Null',
      this._program.programId,
    );
  }

  public verifyMove(
    proof: ProofBytes,
    public_input: PublicMoveInput,
    session_for_account: ActorId | null,
    game_id: ActorId,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'VerifyMove', proof, public_input, session_for_account, game_id],
      '(String, String, ProofBytes, PublicMoveInput, Option<ActorId>, ActorId)',
      'Null',
      this._program.programId,
    );
  }

  public verifyPlacement(
    proof: ProofBytes,
    public_input: PublicStartInput,
    session_for_account: ActorId | null,
    game_id: ActorId,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'VerifyPlacement', proof, public_input, session_for_account, game_id],
      '(String, String, ProofBytes, PublicStartInput, Option<ActorId>, ActorId)',
      'Null',
      this._program.programId,
    );
  }

  public async game(
    player_id: ActorId,
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<MultipleGameState | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry
      .createType('(String, String, ActorId)', ['Multiple', 'Game', player_id])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType('(String, String, Option<MultipleGameState>)', reply.payload);
    return result[2].toJSON() as unknown as MultipleGameState | null;
  }

  public async games(
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, MultipleGameState]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry.createType('(String, String)', '[Multiple, Games]').toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType(
      '(String, String, Vec<(ActorId, MultipleGameState)>)',
      reply.payload,
    );
    return result[2].toJSON() as unknown as Array<[ActorId, MultipleGameState]>;
  }

  public async gamesPairs(
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, ActorId]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry.createType('(String, String)', '[Multiple, GamesPairs]').toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType('(String, String, Vec<(ActorId, ActorId)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, ActorId]>;
  }

  public subscribeToGameCreatedEvent(
    callback: (data: { player_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'GameCreated') {
        callback(
          this._program.registry
            .createType('(String, String, {"player_id":"ActorId"})', message.payload)[2]
            .toJSON() as { player_id: ActorId },
        );
      }
    });
  }

  public subscribeToJoinedTheGameEvent(
    callback: (data: { player_id: ActorId; game_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'JoinedTheGame') {
        callback(
          this._program.registry
            .createType('(String, String, {"player_id":"ActorId","game_id":"ActorId"})', message.payload)[2]
            .toJSON() as { player_id: ActorId; game_id: ActorId },
        );
      }
    });
  }

  public subscribeToPlacementVerifiedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'PlacementVerified') {
        callback(null);
      }
    });
  }

  public subscribeToGameCanceledEvent(
    callback: (data: { game_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'GameCanceled') {
        callback(
          this._program.registry.createType('(String, String, {"game_id":"ActorId"})', message.payload)[2].toJSON() as {
            game_id: ActorId;
          },
        );
      }
    });
  }

  public subscribeToGameLeftEvent(callback: (data: { game_id: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'GameLeft') {
        callback(
          this._program.registry.createType('(String, String, {"game_id":"ActorId"})', message.payload)[2].toJSON() as {
            game_id: ActorId;
          },
        );
      }
    });
  }

  public subscribeToMoveMadeEvent(
    callback: (data: { game_id: ActorId; step: number; target_address: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'MoveMade') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"game_id":"ActorId","step":"u8","target_address":"ActorId"})',
              message.payload,
            )[2]
            .toJSON() as { game_id: ActorId; step: number; target_address: ActorId },
        );
      }
    });
  }

  public subscribeToMoveVerifiedEvent(
    callback: (data: { step: number; result_: number }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'MoveVerified') {
        callback(
          this._program.registry
            .createType('(String, String, {"step":"u8","result_":"u8"})', message.payload)[2]
            .toJSON() as { step: number; result_: number },
        );
      }
    });
  }

  public subscribeToEndGameEvent(
    callback: (data: {
      winner: ActorId;
      total_time: number | string;
      participants_info: Array<[ActorId, ParticipantInfo]>;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'EndGame') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"winner":"ActorId","total_time":"u64","participants_info":"Vec<(ActorId, ParticipantInfo)>"})',
              message.payload,
            )[2]
            .toJSON() as {
            winner: ActorId;
            total_time: number | string;
            participants_info: Array<[ActorId, ParticipantInfo]>;
          },
        );
      }
    });
  }

  public subscribeToGameDeletedEvent(
    callback: (data: { game_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'GameDeleted') {
        callback(
          this._program.registry.createType('(String, String, {"game_id":"ActorId"})', message.payload)[2].toJSON() as {
            game_id: ActorId;
          },
        );
      }
    });
  }

  public subscribeToPlayerDeletedEvent(
    callback: (data: { game_id: ActorId; removable_player: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'PlayerDeleted') {
        callback(
          this._program.registry
            .createType('(String, String, {"game_id":"ActorId","removable_player":"ActorId"})', message.payload)[2]
            .toJSON() as { game_id: ActorId; removable_player: ActorId },
        );
      }
    });
  }
}

export class Session {
  constructor(private _program: Program) {}

  public createSession(
    key: ActorId,
    duration: number | string,
    allowed_actions: Array<ActionsForSession>,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Session', 'CreateSession', key, duration, allowed_actions],
      '(String, String, ActorId, u64, Vec<ActionsForSession>)',
      'Null',
      this._program.programId,
    );
  }

  public deleteSession(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Session', 'DeleteSession'],
      '(String, String)',
      'Null',
      this._program.programId,
    );
  }

  public async sessions(
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, Session]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry.createType('(String, String)', '[Session, Sessions]').toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType('(String, String, Vec<(ActorId, Session)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, Session]>;
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

export class Single {
  constructor(private _program: Program) {}

  public deleteGame(player: ActorId, start_time: number | string): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Single', 'DeleteGame', player, start_time],
      '(String, String, ActorId, u64)',
      'Null',
      this._program.programId,
    );
  }

  public makeMove(step: number, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Single', 'MakeMove', step, session_for_account],
      '(String, String, u8, Option<ActorId>)',
      'Null',
      this._program.programId,
    );
  }

  public startSingleGame(
    proof: ProofBytes,
    public_input: PublicStartInput,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Single', 'StartSingleGame', proof, public_input, session_for_account],
      '(String, String, ProofBytes, PublicStartInput, Option<ActorId>)',
      'Null',
      this._program.programId,
    );
  }

  public verifyMove(
    proof: ProofBytes,
    public_input: PublicMoveInput,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Single', 'VerifyMove', proof, public_input, session_for_account],
      '(String, String, ProofBytes, PublicMoveInput, Option<ActorId>)',
      'Null',
      this._program.programId,
    );
  }

  public async game(
    player_id: ActorId,
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<SingleGame | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry
      .createType('(String, String, ActorId)', ['Single', 'Game', player_id])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType('(String, String, Option<SingleGame>)', reply.payload);
    return result[2].toJSON() as unknown as SingleGame | null;
  }

  public async gameStatus(
    player_id: ActorId,
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<SingleUtilsStatus | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry
      .createType('(String, String, ActorId)', ['Single', 'GameStatus', player_id])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType('(String, String, Option<SingleUtilsStatus>)', reply.payload);
    return result[2].toJSON() as unknown as SingleUtilsStatus | null;
  }

  public async games(
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, SingleGameState]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry.createType('(String, String)', '[Single, Games]').toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType(
      '(String, String, Vec<(ActorId, SingleGameState)>)',
      reply.payload,
    );
    return result[2].toJSON() as unknown as Array<[ActorId, SingleGameState]>;
  }

  public async startTime(
    player_id: ActorId,
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<number | string | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry
      .createType('(String, String, ActorId)', ['Single', 'StartTime', player_id])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType('(String, String, Option<u64>)', reply.payload);
    return result[2].toJSON() as unknown as number | string | null;
  }

  public async totalShots(
    player_id: ActorId,
    originAddress: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<number | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');

    const payload = this._program.registry
      .createType('(String, String, ActorId)', ['Single', 'TotalShots', player_id])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: decodeAddress(originAddress),
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    const result = this._program.registry.createType('(String, String, Option<u8>)', reply.payload);
    return result[2].toJSON() as unknown as number | null;
  }

  public subscribeToSessionCreatedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Single' && getFnNamePrefix(payload) === 'SessionCreated') {
        callback(null);
      }
    });
  }

  public subscribeToSingleGameStartedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Single' && getFnNamePrefix(payload) === 'SingleGameStarted') {
        callback(null);
      }
    });
  }

  public subscribeToEndGameEvent(
    callback: (data: {
      winner: BattleshipParticipants;
      time: number | string;
      total_shots: number;
      succesfull_shots: number;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Single' && getFnNamePrefix(payload) === 'EndGame') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"winner":"BattleshipParticipants","time":"u64","total_shots":"u8","succesfull_shots":"u8"})',
              message.payload,
            )[2]
            .toJSON() as {
            winner: BattleshipParticipants;
            time: number | string;
            total_shots: number;
            succesfull_shots: number;
          },
        );
      }
    });
  }

  public subscribeToMoveMadeEvent(
    callback: (data: { step: number; step_result: StepResult; bot_step: number }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Single' && getFnNamePrefix(payload) === 'MoveMade') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"step":"u8","step_result":"StepResult","bot_step":"u8"})',
              message.payload,
            )[2]
            .toJSON() as { step: number; step_result: StepResult; bot_step: number },
        );
      }
    });
  }

  public subscribeToMoveVerifiedEvent(
    callback: (data: { step: number; result_: number }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Single' && getFnNamePrefix(payload) === 'MoveVerified') {
        callback(
          this._program.registry
            .createType('(String, String, {"step":"u8","result_":"u8"})', message.payload)[2]
            .toJSON() as { step: number; result_: number },
        );
      }
    });
  }
}
