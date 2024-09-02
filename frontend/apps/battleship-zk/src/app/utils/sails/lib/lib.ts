import { TransactionBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';
import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';

export type ActorId = string;

export interface VerifyingKeyBytes {
  alpha_g1_beta_g2: `0x${string}`;
  gamma_g2_neg_pc: `0x${string}`;
  delta_g2_neg_pc: `0x${string}`;
  ic: Array<`0x${string}`>;
}

export interface Configuration {
  gas_for_delete_single_game: number | string | bigint;
  gas_for_delete_multiple_game: number | string | bigint;
  gas_for_check_time: number | string | bigint;
  gas_for_delete_session: number | string | bigint;
  delay_for_delete_single_game: number;
  delay_for_delete_multiple_game: number;
  delay_for_check_time: number;
  minimum_session_duration_ms: number | string | bigint;
  block_duration_ms: number | string | bigint;
}

export interface VerificationVariables {
  proof_bytes: ProofBytes;
  public_input: PublicMoveInput;
}

export interface ProofBytes {
  a: `0x${string}`;
  b: `0x${string}`;
  c: `0x${string}`;
}

export interface PublicMoveInput {
  out: number;
  hit: number;
  hash: `0x${string}`;
}

export interface PublicStartInput {
  hash: `0x${string}`;
}

export interface MultipleGameState {
  admin: ActorId;
  participants_data: Array<[ActorId, ParticipantInfo]>;
  create_time: number | string | bigint;
  start_time: number | string | bigint | null;
  last_move_time: number | string | bigint;
  status: Status;
  bid: number | string | bigint;
}

export interface ParticipantInfo {
  name: string;
  board: Array<Entity>;
  ship_hash: `0x${string}`;
  total_shots: number;
  succesfull_shots: number;
}

export type Entity = 'Empty' | 'Unknown' | 'Occupied' | 'Ship' | 'Boom' | 'BoomShip' | 'DeadShip';

export type Status =
  | { registration: null }
  | { verificationPlacement: ActorId | null }
  | { pendingVerificationOfTheMove: [ActorId, number] }
  | { turn: ActorId };

export type MultipleUtilsStepResult = 'Missed' | 'Injured' | 'Killed';

export interface SignatureData {
  key: ActorId;
  duration: number | string | bigint;
  allowed_actions: Array<ActionsForSession>;
}
export type ActionsForSession = 'playSingleGame' | 'playMultipleGame';

export interface Session {
  key: ActorId;
  expires: number | string | bigint;
  allowed_actions: Array<ActionsForSession>;
  expires_at_block: number;
}

export interface SingleGame {
  player_board: Array<Entity>;
  ship_hash: `0x${string}`;
  bot_ships: Ships;
  start_time: number | string | bigint;
  total_shots: number;
  succesfull_shots: number;
  last_move_time: number | string | bigint;
  verification_requirement: number | null;
}

export interface Ships {
  ship_1: `0x${string}`;
  ship_2: `0x${string}`;
  ship_3: `0x${string}`;
  ship_4: `0x${string}`;
}

export interface SingleGameState {
  player_board: Array<Entity>;
  ship_hash: `0x${string}`;
  start_time: number | string | bigint;
  total_shots: number;
  succesfull_shots: number;
  last_move_time: number | string | bigint;
  verification_requirement: number | null;
}

export type BattleshipParticipants = 'Player' | 'Bot';

export type SingleUtilsStepResult = 'Missed' | 'Injured' | 'Killed';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly admin: Admin;
  public readonly multiple: Multiple;
  public readonly session: Session;
  public readonly single: Single;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
    const types: Record<string, any> = {
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
        gas_for_delete_session: 'u64',
        delay_for_delete_single_game: 'u32',
        delay_for_delete_multiple_game: 'u32',
        delay_for_check_time: 'u32',
        minimum_session_duration_ms: 'u64',
        block_duration_ms: 'u64',
      },
      VerificationVariables: { proof_bytes: 'ProofBytes', public_input: 'PublicMoveInput' },
      ProofBytes: { a: 'Vec<u8>', b: 'Vec<u8>', c: 'Vec<u8>' },
      PublicMoveInput: { out: 'u8', hit: 'u8', hash: 'Vec<u8>' },
      PublicStartInput: { hash: 'Vec<u8>' },
      MultipleGameState: {
        admin: '[u8;32]',
        participants_data: 'Vec<([u8;32], ParticipantInfo)>',
        create_time: 'u64',
        start_time: 'Option<u64>',
        last_move_time: 'u64',
        status: 'Status',
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
      Status: {
        _enum: {
          Registration: 'Null',
          VerificationPlacement: 'Option<[u8;32]>',
          PendingVerificationOfTheMove: '([u8;32], u8)',
          Turn: '[u8;32]',
        },
      },
      MultipleUtilsStepResult: { _enum: ['Missed', 'Injured', 'Killed'] },
      SignatureData: { key: '[u8;32]', duration: 'u64', allowed_actions: 'Vec<ActionsForSession>' },
      ActionsForSession: { _enum: ['PlaySingleGame', 'PlayMultipleGame'] },
      Session: { key: '[u8;32]', expires: 'u64', allowed_actions: 'Vec<ActionsForSession>', expires_at_block: 'u32' },
      SingleGame: {
        player_board: 'Vec<Entity>',
        ship_hash: 'Vec<u8>',
        bot_ships: 'Ships',
        start_time: 'u64',
        total_shots: 'u8',
        succesfull_shots: 'u8',
        last_move_time: 'u64',
        verification_requirement: 'Option<u8>',
      },
      Ships: { ship_1: 'Vec<u8>', ship_2: 'Vec<u8>', ship_3: 'Vec<u8>', ship_4: 'Vec<u8>' },
      SingleGameState: {
        player_board: 'Vec<Entity>',
        ship_hash: 'Vec<u8>',
        start_time: 'u64',
        total_shots: 'u8',
        succesfull_shots: 'u8',
        last_move_time: 'u64',
        verification_requirement: 'Option<u8>',
      },
      BattleshipParticipants: { _enum: ['Player', 'Bot'] },
      SingleUtilsStepResult: { _enum: ['Missed', 'Injured', 'Killed'] },
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
      '(String, [u8;32], VerifyingKeyBytes, VerifyingKeyBytes, Configuration)',
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
      '(String, [u8;32], VerifyingKeyBytes, VerifyingKeyBytes, Configuration)',
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
      '(String, String, [u8;32])',
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
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public changeConfiguration(configuration: Configuration): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Admin', 'ChangeConfiguration', configuration],
      '(String, String, Configuration)',
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
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public deleteMultipleGamesByTime(time: number | string | bigint): TransactionBuilder<null> {
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

  public deleteMultipleGamesInBatches(divider: number | string | bigint): TransactionBuilder<null> {
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
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public deleteSingleGames(time: number | string | bigint): TransactionBuilder<null> {
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
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public async admin(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<ActorId> {
    const payload = this._program.registry.createType('(String, String)', ['Admin', 'Admin']).toHex();
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
    return result[2].toJSON() as unknown as ActorId;
  }

  public async builtin(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<ActorId> {
    const payload = this._program.registry.createType('(String, String)', ['Admin', 'Builtin']).toHex();
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
    return result[2].toJSON() as unknown as ActorId;
  }

  public async configuration(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Configuration> {
    const payload = this._program.registry.createType('(String, String)', ['Admin', 'Configuration']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Configuration)', reply.payload);
    return result[2].toJSON() as unknown as Configuration;
  }

  public async verificationKey(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<[VerifyingKeyBytes, VerifyingKeyBytes]> {
    const payload = this._program.registry.createType('(String, String)', ['Admin', 'VerificationKey']).toHex();
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

  public subscribeToConfigurationChangedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Admin' && getFnNamePrefix(payload) === 'ConfigurationChanged') {
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
            .createType('(String, String, {"inheritor":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { inheritor: ActorId },
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
      '(String, String, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public checkOutTiming(game_id: ActorId, check_time: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'CheckOutTiming', game_id, check_time],
      '(String, String, [u8;32], u64)',
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
      '(String, String, String, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public deleteGame(game_id: ActorId, create_time: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'DeleteGame', game_id, create_time],
      '(String, String, [u8;32], u64)',
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
      '(String, String, [u8;32], Option<[u8;32]>)',
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
      '(String, String, [u8;32], String, Option<[u8;32]>)',
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
      '(String, String, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public makeMove(
    game_id: ActorId,
    verify_variables: VerificationVariables | null,
    step: number | null,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Multiple', 'MakeMove', game_id, verify_variables, step, session_for_account],
      '(String, String, [u8;32], Option<VerificationVariables>, Option<u8>, Option<[u8;32]>)',
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
      '(String, String, ProofBytes, PublicStartInput, Option<[u8;32]>, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public async game(
    player_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<MultipleGameState | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Multiple', 'Game', player_id])
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
    const result = this._program.registry.createType('(String, String, Option<MultipleGameState>)', reply.payload);
    return result[2].toJSON() as unknown as MultipleGameState | null;
  }

  public async games(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, MultipleGameState]>> {
    const payload = this._program.registry.createType('(String, String)', ['Multiple', 'Games']).toHex();
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
      '(String, String, Vec<([u8;32], MultipleGameState)>)',
      reply.payload,
    );
    return result[2].toJSON() as unknown as Array<[ActorId, MultipleGameState]>;
  }

  public async gamesPairs(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, ActorId]>> {
    const payload = this._program.registry.createType('(String, String)', ['Multiple', 'GamesPairs']).toHex();
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

  public async getRemainingTime(
    player_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<number | string | bigint | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Multiple', 'GetRemainingTime', player_id])
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
    const result = this._program.registry.createType('(String, String, Option<u64>)', reply.payload);
    return result[2].toJSON() as unknown as number | string | bigint | null;
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
            .createType('(String, String, {"player_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { player_id: ActorId },
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
            .createType('(String, String, {"player_id":"[u8;32]","game_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { player_id: ActorId; game_id: ActorId },
        );
      }
    });
  }

  public subscribeToPlacementVerifiedEvent(
    callback: (data: { admin: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Multiple' && getFnNamePrefix(payload) === 'PlacementVerified') {
        callback(
          this._program.registry
            .createType('(String, String, {"admin":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { admin: ActorId },
        );
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
          this._program.registry
            .createType('(String, String, {"game_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { game_id: ActorId },
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
          this._program.registry
            .createType('(String, String, {"game_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { game_id: ActorId },
        );
      }
    });
  }

  public subscribeToMoveMadeEvent(
    callback: (data: {
      game_id: ActorId;
      step: number | null;
      verified_result: [number, MultipleUtilsStepResult] | null;
      turn: ActorId;
    }) => void | Promise<void>,
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
              '(String, String, {"game_id":"[u8;32]","step":"Option<u8>","verified_result":"Option<(u8, MultipleUtilsStepResult)>","turn":"[u8;32]"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            game_id: ActorId;
            step: number | null;
            verified_result: [number, MultipleUtilsStepResult] | null;
            turn: ActorId;
          },
        );
      }
    });
  }

  public subscribeToEndGameEvent(
    callback: (data: {
      admin: ActorId;
      winner: ActorId;
      total_time: number | string | bigint;
      participants_info: Array<[ActorId, ParticipantInfo]>;
      last_hit: number | null;
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
              '(String, String, {"admin":"[u8;32]","winner":"[u8;32]","total_time":"u64","participants_info":"Vec<([u8;32], ParticipantInfo)>","last_hit":"Option<u8>"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            admin: ActorId;
            winner: ActorId;
            total_time: number | string | bigint;
            participants_info: Array<[ActorId, ParticipantInfo]>;
            last_hit: number | null;
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
          this._program.registry
            .createType('(String, String, {"game_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { game_id: ActorId },
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
            .createType('(String, String, {"game_id":"[u8;32]","removable_player":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { game_id: ActorId; removable_player: ActorId },
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
  ): Promise<Session | null> {
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
    const result = this._program.registry.createType('(String, String, Option<Session>)', reply.payload);
    return result[2].toJSON() as unknown as Session | null;
  }

  public async sessions(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, Session]>> {
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
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], Session)>)', reply.payload);
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

  public checkOutTiming(actor_id: ActorId, check_time: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Single', 'CheckOutTiming', actor_id, check_time],
      '(String, String, [u8;32], u64)',
      'Null',
      this._program.programId,
    );
  }

  public deleteGame(player: ActorId, start_time: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Single', 'DeleteGame', player, start_time],
      '(String, String, [u8;32], u64)',
      'Null',
      this._program.programId,
    );
  }

  public makeMove(
    step: number | null,
    verify_variables: VerificationVariables | null,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Single', 'MakeMove', step, verify_variables, session_for_account],
      '(String, String, Option<u8>, Option<VerificationVariables>, Option<[u8;32]>)',
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
      '(String, String, ProofBytes, PublicStartInput, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public async game(
    player_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<SingleGame | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Single', 'Game', player_id])
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
    const result = this._program.registry.createType('(String, String, Option<SingleGame>)', reply.payload);
    return result[2].toJSON() as unknown as SingleGame | null;
  }

  public async games(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, SingleGameState]>> {
    const payload = this._program.registry.createType('(String, String)', ['Single', 'Games']).toHex();
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
      '(String, String, Vec<([u8;32], SingleGameState)>)',
      reply.payload,
    );
    return result[2].toJSON() as unknown as Array<[ActorId, SingleGameState]>;
  }

  public async getRemainingTime(
    player_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<number | string | bigint | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Single', 'GetRemainingTime', player_id])
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
    const result = this._program.registry.createType('(String, String, Option<u64>)', reply.payload);
    return result[2].toJSON() as unknown as number | string | bigint | null;
  }

  public async startTime(
    player_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<number | string | bigint | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Single', 'StartTime', player_id])
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
    const result = this._program.registry.createType('(String, String, Option<u64>)', reply.payload);
    return result[2].toJSON() as unknown as number | string | bigint | null;
  }

  public async totalShots(
    player_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<number | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Single', 'TotalShots', player_id])
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
      player: ActorId;
      winner: BattleshipParticipants;
      time: number | string | bigint;
      total_shots: number;
      succesfull_shots: number;
      last_hit: number | null;
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
              '(String, String, {"player":"[u8;32]","winner":"BattleshipParticipants","time":"u64","total_shots":"u8","succesfull_shots":"u8","last_hit":"Option<u8>"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            player: ActorId;
            winner: BattleshipParticipants;
            time: number | string | bigint;
            total_shots: number;
            succesfull_shots: number;
            last_hit: number | null;
          },
        );
      }
    });
  }

  public subscribeToMoveMadeEvent(
    callback: (data: {
      player: ActorId;
      step: number | null;
      step_result: SingleUtilsStepResult | null;
      bot_step: number | null;
    }) => void | Promise<void>,
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
              '(String, String, {"player":"[u8;32]","step":"Option<u8>","step_result":"Option<SingleUtilsStepResult>","bot_step":"Option<u8>"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            player: ActorId;
            step: number | null;
            step_result: SingleUtilsStepResult | null;
            bot_step: number | null;
          },
        );
      }
    });
  }
}
