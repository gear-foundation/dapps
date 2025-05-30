/* eslint-disable */
import { GearApi, HexString, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import {
  TransactionBuilder,
  ActorId,
  throwOnErrorReply,
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
        small_blind: 'u128',
        big_blind: 'u128',
        number_of_participants: 'u16',
        starting_bank: 'u128',
      },
      PublicKey: { x: '[u8; 32]', y: '[u8; 32]', z: '[u8; 32]' },
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
    vk_shuffle_bytes: `0x${string}`,
    vk_decrypt_bytes: `0x${string}`,
  ): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', config, pts_actor_id, vk_shuffle_bytes, vk_decrypt_bytes],
      '(String, Config, [u8;32], Vec<u8>, Vec<u8>)',
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
    vk_shuffle_bytes: `0x${string}`,
    vk_decrypt_bytes: `0x${string}`,
  ) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', config, pts_actor_id, vk_shuffle_bytes, vk_decrypt_bytes],
      '(String, Config, [u8;32], Vec<u8>, Vec<u8>)',
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
      ['PokerFactory', 'AddAdmin', new_admin_id],
      '(String, String, [u8;32])',
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
  public createLobby(init_lobby: LobbyConfig, pk: PublicKey): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['PokerFactory', 'CreateLobby', init_lobby, pk],
      '(String, String, LobbyConfig, PublicKey)',
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
      ['PokerFactory', 'DeleteAdmin', id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Deletes lobby from registry. Admin or lobby itself only.
   * Panics if:
   * - Lobby doesn't exist
   * - Caller lacks permissions
   * Emits LobbyDeleted event on success.
   */
  public deleteLobby(lobby_address: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['PokerFactory', 'DeleteLobby', lobby_address],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public async admins(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<ActorId>> {
    const payload = this._program.registry.createType('(String, String)', ['PokerFactory', 'Admins']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<[u8;32]>)', reply.payload);
    return result[2].toJSON() as unknown as Array<ActorId>;
  }

  public async config(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<PokerFactoryConfig> {
    const payload = this._program.registry.createType('(String, String)', ['PokerFactory', 'Config']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Config)', reply.payload);
    return result[2].toJSON() as unknown as PokerFactoryConfig;
  }

  public async lobbies(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, LobbyConfig]>> {
    const payload = this._program.registry.createType('(String, String)', ['PokerFactory', 'Lobbies']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], LobbyConfig)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, LobbyConfig]>;
  }

  public async ptsActorId(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<ActorId> {
    const payload = this._program.registry.createType('(String, String)', ['PokerFactory', 'PtsActorId']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, [u8;32])', reply.payload);
    return result[2].toJSON() as unknown as ActorId;
  }

  public async vkDecryptBytes(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<`0x${string}`> {
    const payload = this._program.registry.createType('(String, String)', ['PokerFactory', 'VkDecryptBytes']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<u8>)', reply.payload);
    return result[2].toJSON() as unknown as `0x${string}`;
  }

  public async vkShuffleBytes(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<`0x${string}`> {
    const payload = this._program.registry.createType('(String, String)', ['PokerFactory', 'VkShuffleBytes']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<u8>)', reply.payload);
    return result[2].toJSON() as unknown as `0x${string}`;
  }

  public subscribeToLobbyCreatedEvent(
    callback: (data: { lobby_address: ActorId; admin: ActorId; lobby_config: LobbyConfig }) => void | Promise<void>,
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
              '(String, String, {"lobby_address":"[u8;32]","admin":"[u8;32]","lobby_config":"LobbyConfig"})',
              message.payload,
            )[2]
            .toJSON() as unknown as { lobby_address: ActorId; admin: ActorId; lobby_config: LobbyConfig },
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
