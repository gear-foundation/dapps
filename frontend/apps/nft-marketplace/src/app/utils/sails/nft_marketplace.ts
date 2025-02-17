import { GearApi, HexString, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

type ActorId = HexString;

export interface MarketState {
  admin_id: ActorId;
  items: Array<[[ActorId, number | string | bigint], ItemState]>;
  approved_nft_contracts: Array<ActorId>;
  approved_ft_contracts: Array<ActorId>;
}

export interface ItemState {
  frozen: boolean;
  token_id: number | string | bigint;
  owner: ActorId;
  ft_contract_id: ActorId | null;
  price: number | string | bigint | null;
  auction: Auction | null;
  offers: Array<[[ActorId | null, number | string | bigint], ActorId]>;
}

export interface Auction {
  started_at: number | string | bigint;
  ended_at: number | string | bigint;
  current_price: number | string | bigint;
  current_winner: ActorId;
}

export class Program {
  public readonly registry: TypeRegistry;
  public readonly nftMarketplace: NftMarketplace;

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      MarketState: {
        admin_id: '[u8;32]',
        items: 'Vec<(([u8;32], U256), ItemState)>',
        approved_nft_contracts: 'Vec<[u8;32]>',
        approved_ft_contracts: 'Vec<[u8;32]>',
      },
      ItemState: {
        frozen: 'bool',
        token_id: 'U256',
        owner: '[u8;32]',
        ft_contract_id: 'Option<[u8;32]>',
        price: 'Option<u128>',
        auction: 'Option<Auction>',
        offers: 'Vec<((Option<[u8;32]>, u128), [u8;32])>',
      },
      Auction: { started_at: 'u64', ended_at: 'u64', current_price: 'u128', current_winner: '[u8;32]' },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.nftMarketplace = new NftMarketplace(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, admin_id: ActorId): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', admin_id],
      '(String, [u8;32])',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, admin_id: ActorId) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', admin_id],
      '(String, [u8;32])',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class NftMarketplace {
  constructor(private _program: Program) {}

  public acceptOffer(
    nft_contract_id: ActorId,
    ft_contract_id: ActorId | null,
    token_id: number | string | bigint,
    price: number | string | bigint,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'AcceptOffer', nft_contract_id, ft_contract_id, token_id, price],
      '(String, String, [u8;32], Option<[u8;32]>, U256, u128)',
      'Null',
      this._program.programId,
    );
  }

  public addBid(
    nft_contract_id: ActorId,
    token_id: number | string | bigint,
    price: number | string | bigint,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'AddBid', nft_contract_id, token_id, price],
      '(String, String, [u8;32], U256, u128)',
      'Null',
      this._program.programId,
    );
  }

  public addFtContract(ft_contract_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'AddFtContract', ft_contract_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public addMarketData(
    nft_contract_id: ActorId,
    ft_contract_id: ActorId | null,
    token_id: number | string | bigint,
    price: number | string | bigint | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'AddMarketData', nft_contract_id, ft_contract_id, token_id, price],
      '(String, String, [u8;32], Option<[u8;32]>, U256, Option<u128>)',
      'Null',
      this._program.programId,
    );
  }

  public addNftContract(nft_contract_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'AddNftContract', nft_contract_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public addOffer(
    nft_contract_id: ActorId,
    ft_contract_id: ActorId | null,
    token_id: number | string | bigint,
    price: number | string | bigint,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'AddOffer', nft_contract_id, ft_contract_id, token_id, price],
      '(String, String, [u8;32], Option<[u8;32]>, U256, u128)',
      'Null',
      this._program.programId,
    );
  }

  public buyItem(nft_contract_id: ActorId, token_id: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'BuyItem', nft_contract_id, token_id],
      '(String, String, [u8;32], U256)',
      'Null',
      this._program.programId,
    );
  }

  public createAuction(
    nft_contract_id: ActorId,
    ft_contract_id: ActorId | null,
    token_id: number | string | bigint,
    min_price: number | string | bigint,
    duration: number | string | bigint,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'CreateAuction', nft_contract_id, ft_contract_id, token_id, min_price, duration],
      '(String, String, [u8;32], Option<[u8;32]>, U256, u128, u64)',
      'Null',
      this._program.programId,
    );
  }

  public removeMarketData(nft_contract_id: ActorId, token_id: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'RemoveMarketData', nft_contract_id, token_id],
      '(String, String, [u8;32], U256)',
      'Null',
      this._program.programId,
    );
  }

  public settleAuction(nft_contract_id: ActorId, token_id: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'SettleAuction', nft_contract_id, token_id],
      '(String, String, [u8;32], U256)',
      'Null',
      this._program.programId,
    );
  }

  public withdraw(
    nft_contract_id: ActorId,
    ft_contract_id: ActorId | null,
    token_id: number | string | bigint,
    price: number | string | bigint,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['NftMarketplace', 'Withdraw', nft_contract_id, ft_contract_id, token_id, price],
      '(String, String, [u8;32], Option<[u8;32]>, U256, u128)',
      'Null',
      this._program.programId,
    );
  }

  public async getMarket(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<MarketState> {
    const payload = this._program.registry.createType('(String, String)', ['NftMarketplace', 'GetMarket']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, MarketState)', reply.payload);
    return result[2].toJSON() as unknown as MarketState;
  }

  public subscribeToNftContractAddedEvent(callback: (data: ActorId) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'NftContractAdded') {
        callback(
          this._program.registry
            .createType('(String, String, [u8;32])', message.payload)[2]
            .toJSON() as unknown as ActorId,
        );
      }
    });
  }

  public subscribeToFtContractAddedEvent(callback: (data: ActorId) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'FtContractAdded') {
        callback(
          this._program.registry
            .createType('(String, String, [u8;32])', message.payload)[2]
            .toJSON() as unknown as ActorId,
        );
      }
    });
  }

  public subscribeToMarketDataAddedEvent(
    callback: (data: {
      nft_contract_id: ActorId;
      token_id: number | string | bigint;
      price: number | string | bigint | null;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'MarketDataAdded') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"nft_contract_id":"[u8;32]","token_id":"U256","price":"Option<u128>"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            nft_contract_id: ActorId;
            token_id: number | string | bigint;
            price: number | string | bigint | null;
          },
        );
      }
    });
  }

  public subscribeToMarketDataRemovedEvent(
    callback: (data: {
      owner: ActorId;
      nft_contract_id: ActorId;
      token_id: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'MarketDataRemoved') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"owner":"[u8;32]","nft_contract_id":"[u8;32]","token_id":"U256"})',
              message.payload,
            )[2]
            .toJSON() as unknown as { owner: ActorId; nft_contract_id: ActorId; token_id: number | string | bigint },
        );
      }
    });
  }

  public subscribeToItemSoldEvent(
    callback: (data: {
      owner: ActorId;
      nft_contract_id: ActorId;
      token_id: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'ItemSold') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"owner":"[u8;32]","nft_contract_id":"[u8;32]","token_id":"U256"})',
              message.payload,
            )[2]
            .toJSON() as unknown as { owner: ActorId; nft_contract_id: ActorId; token_id: number | string | bigint },
        );
      }
    });
  }

  public subscribeToBidAddedEvent(
    callback: (data: {
      nft_contract_id: ActorId;
      token_id: number | string | bigint;
      price: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'BidAdded') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"nft_contract_id":"[u8;32]","token_id":"U256","price":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            nft_contract_id: ActorId;
            token_id: number | string | bigint;
            price: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToAuctionCreatedEvent(
    callback: (data: {
      nft_contract_id: ActorId;
      token_id: number | string | bigint;
      price: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'AuctionCreated') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"nft_contract_id":"[u8;32]","token_id":"U256","price":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            nft_contract_id: ActorId;
            token_id: number | string | bigint;
            price: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToAuctionSettledEvent(
    callback: (data: {
      nft_contract_id: ActorId;
      token_id: number | string | bigint;
      price: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'AuctionSettled') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"nft_contract_id":"[u8;32]","token_id":"U256","price":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            nft_contract_id: ActorId;
            token_id: number | string | bigint;
            price: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToAuctionCancelledEvent(
    callback: (data: { nft_contract_id: ActorId; token_id: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'AuctionCancelled') {
        callback(
          this._program.registry
            .createType('(String, String, {"nft_contract_id":"[u8;32]","token_id":"U256"})', message.payload)[2]
            .toJSON() as unknown as { nft_contract_id: ActorId; token_id: number | string | bigint },
        );
      }
    });
  }

  public subscribeToNFTListedEvent(
    callback: (data: {
      nft_contract_id: ActorId;
      owner: ActorId;
      token_id: number | string | bigint;
      price: number | string | bigint | null;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'NFTListed') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"nft_contract_id":"[u8;32]","owner":"[u8;32]","token_id":"U256","price":"Option<u128>"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            nft_contract_id: ActorId;
            owner: ActorId;
            token_id: number | string | bigint;
            price: number | string | bigint | null;
          },
        );
      }
    });
  }

  public subscribeToOfferAddedEvent(
    callback: (data: {
      nft_contract_id: ActorId;
      ft_contract_id: ActorId | null;
      token_id: number | string | bigint;
      price: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'OfferAdded') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"nft_contract_id":"[u8;32]","ft_contract_id":"Option<[u8;32]>","token_id":"U256","price":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            nft_contract_id: ActorId;
            ft_contract_id: ActorId | null;
            token_id: number | string | bigint;
            price: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToOfferAcceptedEvent(
    callback: (data: {
      nft_contract_id: ActorId;
      token_id: number | string | bigint;
      new_owner: ActorId;
      price: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'OfferAccepted') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"nft_contract_id":"[u8;32]","token_id":"U256","new_owner":"[u8;32]","price":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            nft_contract_id: ActorId;
            token_id: number | string | bigint;
            new_owner: ActorId;
            price: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToWithdrawEvent(
    callback: (data: {
      nft_contract_id: ActorId;
      token_id: number | string | bigint;
      price: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'Withdraw') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"nft_contract_id":"[u8;32]","token_id":"U256","price":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            nft_contract_id: ActorId;
            token_id: number | string | bigint;
            price: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToTransactionFailedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'TransactionFailed') {
        callback(null);
      }
    });
  }

  public subscribeToRerunTransactionEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'RerunTransaction') {
        callback(null);
      }
    });
  }

  public subscribeToTransferValueEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'NftMarketplace' && getFnNamePrefix(payload) === 'TransferValue') {
        callback(null);
      }
    });
  }
}
