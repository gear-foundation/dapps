/* eslint-disable */
import { GearApi, HexString } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, QueryBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

export interface TokenMetadata {
  name: string;
  description: string;
  media: string;
  reference: string;
}

type ActorId = HexString;

export class Program {
  public readonly registry: TypeRegistry;
  public readonly vnft: Vnft;

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      TokenMetadata: { name: 'String', description: 'String', media: 'String', reference: 'String' },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.vnft = new Vnft(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, name: string, symbol: string): TransactionBuilder<null> {
    // @ts-ignore
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      [name, symbol],
      '(String, String)',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, name: string, symbol: string) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      null,
      'New',
      [name, symbol],
      '(String, String)',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class Vnft {
  constructor(private _program: Program) {}

  public burn(from: ActorId, token_id: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'Burn',
      [from, token_id],
      '([u8;32], U256)',
      'Null',
      this._program.programId,
    );
  }

  public grantAdminRole(to: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'GrantAdminRole',
      to,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public grantBurnerRole(to: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'GrantBurnerRole',
      to,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public grantMinterRole(to: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'GrantMinterRole',
      to,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public mint(to: ActorId, token_metadata: TokenMetadata): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'Mint',
      [to, token_metadata],
      '([u8;32], TokenMetadata)',
      'Null',
      this._program.programId,
    );
  }

  public revokeAdminRole(from: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'RevokeAdminRole',
      from,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public revokeBurnerRole(from: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'RevokeBurnerRole',
      from,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public revokeMinterRole(from: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'RevokeMinterRole',
      from,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public approve(approved: ActorId, token_id: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'Approve',
      [approved, token_id],
      '([u8;32], U256)',
      'Null',
      this._program.programId,
    );
  }

  public transfer(to: ActorId, token_id: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'Transfer',
      [to, token_id],
      '([u8;32], U256)',
      'Null',
      this._program.programId,
    );
  }

  public transferFrom(from: ActorId, to: ActorId, token_id: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Vnft',
      'TransferFrom',
      [from, to, token_id],
      '([u8;32], [u8;32], U256)',
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
      'Vnft',
      'Admins',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public burners(): QueryBuilder<Array<ActorId>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'Burners',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public minters(): QueryBuilder<Array<ActorId>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'Minters',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public tokenId(): QueryBuilder<bigint> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<bigint>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'TokenId',
      null,
      null,
      'U256',
    );
  }

  public tokenMetadataById(token_id: number | string | bigint): QueryBuilder<TokenMetadata | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<TokenMetadata | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'TokenMetadataById',
      token_id,
      'U256',
      'Option<TokenMetadata>',
    );
  }

  public tokensForOwner(owner: ActorId): QueryBuilder<Array<[number | string | bigint, TokenMetadata]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<[number | string | bigint, TokenMetadata]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'TokensForOwner',
      owner,
      '[u8;32]',
      'Vec<(U256, TokenMetadata)>',
    );
  }

  public balanceOf(owner: ActorId): QueryBuilder<bigint> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<bigint>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'BalanceOf',
      owner,
      '[u8;32]',
      'U256',
    );
  }

  public getApproved(token_id: number | string | bigint): QueryBuilder<ActorId> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<ActorId>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'GetApproved',
      token_id,
      'U256',
      '[u8;32]',
    );
  }

  public name(): QueryBuilder<string> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<string>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'Name',
      null,
      null,
      'String',
    );
  }

  public ownerOf(token_id: number | string | bigint): QueryBuilder<ActorId> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<ActorId>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'OwnerOf',
      token_id,
      'U256',
      '[u8;32]',
    );
  }

  public symbol(): QueryBuilder<string> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<string>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Vnft',
      'Symbol',
      null,
      null,
      'String',
    );
  }

  public subscribeToMintedEvent(
    callback: (data: { to: ActorId; token_metadata: TokenMetadata }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Vnft' && getFnNamePrefix(payload) === 'Minted') {
        callback(
          this._program.registry
            .createType('(String, String, {"to":"[u8;32]","token_metadata":"TokenMetadata"})', message.payload)[2]
            .toJSON() as unknown as { to: ActorId; token_metadata: TokenMetadata },
        );
      }
    });
  }

  public subscribeToBurnedEvent(
    callback: (data: { from: ActorId; token_id: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Vnft' && getFnNamePrefix(payload) === 'Burned') {
        callback(
          this._program.registry
            .createType('(String, String, {"from":"[u8;32]","token_id":"U256"})', message.payload)[2]
            .toJSON() as unknown as { from: ActorId; token_id: number | string | bigint },
        );
      }
    });
  }

  public subscribeToTransferEvent(
    callback: (data: { from: ActorId; to: ActorId; token_id: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Vnft' && getFnNamePrefix(payload) === 'Transfer') {
        callback(
          this._program.registry
            .createType('(String, String, {"from":"[u8;32]","to":"[u8;32]","token_id":"U256"})', message.payload)[2]
            .toJSON() as unknown as { from: ActorId; to: ActorId; token_id: number | string | bigint },
        );
      }
    });
  }

  public subscribeToApprovalEvent(
    callback: (data: { owner: ActorId; approved: ActorId; token_id: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Vnft' && getFnNamePrefix(payload) === 'Approval') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"owner":"[u8;32]","approved":"[u8;32]","token_id":"U256"})',
              message.payload,
            )[2]
            .toJSON() as unknown as { owner: ActorId; approved: ActorId; token_id: number | string | bigint },
        );
      }
    });
  }
}
