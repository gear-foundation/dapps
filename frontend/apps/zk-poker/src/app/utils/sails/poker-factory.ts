/* eslint-disable */
import { GearApi, HexString } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import {
  QueryBuilder,
  TransactionBuilder,
  ActorId,
  getServiceNamePrefix,
  getFnNamePrefix,
  ZERO_ADDRESS,
} from 'sails-js';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly pokerFactory: PokerFactory;

  constructor(
    public api: GearApi,
    private _programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      Config: { lobby_code_id: '[u8;32]', gas_for_program: 'u64', gas_for_reply_deposit: 'u64' },
      LobbyConfig: {
        admin_id: '[u8;32]',
        admin_name: 'String',
        lobby_name: 'String',
        starting_bank: 'u128',
        time_per_move_ms: 'u64',
        revival: 'bool',
        lobby_time_limit_ms: 'Option<u64>',
        time_until_start_ms: 'Option<u64>',
      },
      ZkPublicKey: { x: '[u8; 32]', y: '[u8; 32]', z: '[u8; 32]' },
      SignatureInfo: { signature_data: 'SignatureData', signature: 'Option<Vec<u8>>' },
      SignatureData: { key: '[u8;32]', duration: 'u64', allowed_actions: 'Vec<ActionsForSession>' },
      ActionsForSession: { _enum: ['AllActions'] },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.pokerFactory = new PokerFactory(this);
  }

  public get programId(): `0x${string}` {
    if (!this._programId) throw new Error(`Program ID is not set`);
    return this._programId;
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
    );

    this._programId = builder.programId;
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
    );

    this._programId = builder.programId;
    return builder;
  }
}

export class PokerFactory {
  constructor(private _program: Program) {}

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

  public changeConfig(config: Config): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'PokerFactory',
      'ChangeConfig',
      config,
      'Config',
      'Null',
      this._program.programId,
    );
  }

  public changePtsActorId(pts_actor_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'PokerFactory',
      'ChangePtsActorId',
      pts_actor_id,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public changeZkVerificationId(zk_verification_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'PokerFactory',
      'ChangeZkVerificationId',
      zk_verification_id,
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
    if (!this._program.programId) throw new Error('Program ID is not set');
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
    if (!this._program.programId) throw new Error('Program ID is not set');
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
    if (!this._program.programId) throw new Error('Program ID is not set');
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
    if (!this._program.programId) throw new Error('Program ID is not set');
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

  public subscribeToConfigChangedEvent(
    callback: (data: { config: Config }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'PokerFactory' && getFnNamePrefix(payload) === 'ConfigChanged') {
        callback(
          this._program.registry
            .createType('(String, String, {"config":"Config"})', message.payload)[2]
            .toJSON() as unknown as { config: Config },
        );
      }
    });
  }

  public subscribeToZkVerificationIdChangedEvent(
    callback: (data: { zk_verification_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'PokerFactory' && getFnNamePrefix(payload) === 'ZkVerificationIdChanged') {
        callback(
          this._program.registry
            .createType('(String, String, {"zk_verification_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { zk_verification_id: ActorId },
        );
      }
    });
  }

  public subscribeToPtsActorIdChangedEvent(
    callback: (data: { pts_actor_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'PokerFactory' && getFnNamePrefix(payload) === 'PtsActorIdChanged') {
        callback(
          this._program.registry
            .createType('(String, String, {"pts_actor_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { pts_actor_id: ActorId },
        );
      }
    });
  }
}
