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
  public readonly pokerFactory: PokerFactory;
  private _program?: BaseGearProgram;

  constructor(
    public api: GearApi,
    programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      Config: { lobby_code_id: '[u8;32]', gas_for_program: 'u64', gas_for_reply_deposit: 'u64' },
      LobbyConfig: {
        admin_id: '[u8;32]',
        admin_name: 'String',
        lobby_name: 'String',
        small_blind: 'u128',
        big_blind: 'u128',
        starting_bank: 'u128',
        time_per_move_ms: 'u64',
      },
      ZkPublicKey: { x: '[u8; 32]', y: '[u8; 32]', z: '[u8; 32]' },
      SignatureInfo: { signature_data: 'SignatureData', signature: 'Option<Vec<u8>>' },
      SignatureData: { key: '[u8;32]', duration: 'u64', allowed_actions: 'Vec<ActionsForSession>' },
      ActionsForSession: { _enum: ['AllActions'] },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);
    if (programId) {
      this._program = new BaseGearProgram(programId, api);
    }

    this.pokerFactory = new PokerFactory(this);
  }

  public get programId(): `0x${string}` {
    if (!this._program) throw new Error(`Program ID is not set`);
    return this._program.id;
  }

  newCtorFromCode(
    code: Uint8Array | Buffer | HexString,
    config: PokerFactoryConfig,
    pts_actor_id: ActorId,
    zk_verification_id: ActorId,
  ): TransactionBuilder<null> {
    // @ts-ignore
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      [config, pts_actor_id, zk_verification_id],
      '(Config, [u8;32], [u8;32])',
      'String',
      code,
      async (programId) => {
        this._program = await BaseGearProgram.new(programId, this.api);
      },
    );
    return builder;
  }

  newCtorFromCodeId(
    codeId: `0x${string}`,
    config: PokerFactoryConfig,
    pts_actor_id: ActorId,
    zk_verification_id: ActorId,
  ) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      null,
      'New',
      [config, pts_actor_id, zk_verification_id],
      '(Config, [u8;32], [u8;32])',
      'String',
      codeId,
      async (programId) => {
        this._program = await BaseGearProgram.new(programId, this.api);
      },
    );
    return builder;
  }
}

export class PokerFactory {
  constructor(private _program: SailsProgram) {}

  public addAdmin(new_admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'PokerFactory',
      'AddAdmin',
      new_admin_id,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Creates new poker lobby with provided config.
   *
   * Panics if:
   * - Insufficient PTS balance
   * - Program creation fails
   *
   * Performs:
   * 1. Checks player's PTS balance
   * 2. Deploys new lobby program
   * 3. Sets lobby as PTS admin
   * 4. Transfers starting bank to lobby
   * 5. Stores lobby info and emits LobbyCreated event
   */
  public createLobby(
    init_lobby: LobbyConfig,
    pk: ZkPublicKey,
    session: SignatureInfo | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'PokerFactory',
      'CreateLobby',
      [init_lobby, pk, session],
      '(LobbyConfig, ZkPublicKey, Option<SignatureInfo>)',
      'Null',
      this._program.programId,
    );
  }

  public deleteAdmin(id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'PokerFactory',
      'DeleteAdmin',
      id,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Deletes lobby from registry. Admin or lobby itself only.
   * Panics if:
   * - Lobby doesn't exist
   * - Caller lacks permissions
   *
   * Emits LobbyDeleted event on success.
   */
  public deleteLobby(lobby_address: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'PokerFactory',
      'DeleteLobby',
      lobby_address,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public admins(): QueryBuilder<Array<ActorId>> {
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'PokerFactory',
      'Admins',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public config(): QueryBuilder<PokerFactoryConfig> {
    return new QueryBuilder<PokerFactoryConfig>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'PokerFactory',
      'Config',
      null,
      null,
      'Config',
    );
  }

  public lobbies(): QueryBuilder<Array<[ActorId, LobbyConfig]>> {
    return new QueryBuilder<Array<[ActorId, LobbyConfig]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'PokerFactory',
      'Lobbies',
      null,
      null,
      'Vec<([u8;32], LobbyConfig)>',
    );
  }

  public ptsActorId(): QueryBuilder<ActorId> {
    return new QueryBuilder<ActorId>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'PokerFactory',
      'PtsActorId',
      null,
      null,
      '[u8;32]',
    );
  }

  public subscribeToLobbyCreatedEvent(
    callback: (data: {
      lobby_address: ActorId;
      admin: ActorId;
      pk: ZkPublicKey;
      lobby_config: LobbyConfig;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'PokerFactory' && getFnNamePrefix(payload) === 'LobbyCreated') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"lobby_address":"[u8;32]","admin":"[u8;32]","pk":"ZkPublicKey","lobby_config":"LobbyConfig"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            lobby_address: ActorId;
            admin: ActorId;
            pk: ZkPublicKey;
            lobby_config: LobbyConfig;
          },
        );
      }
    });
  }

  public subscribeToLobbyDeletedEvent(
    callback: (data: { lobby_address: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'PokerFactory' && getFnNamePrefix(payload) === 'LobbyDeleted') {
        callback(
          this._program.registry
            .createType('(String, String, {"lobby_address":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { lobby_address: ActorId },
        );
      }
    });
  }
}
