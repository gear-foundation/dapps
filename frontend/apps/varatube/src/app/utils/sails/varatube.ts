import { TransactionBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';
import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';

type ActorId = string;

export interface Config {
  gas_for_token_transfer: number | string | bigint;
  gas_to_start_subscription_update: number | string | bigint;
  block_duration: number;
  min_gas_limit: number | string | bigint;
}

export type Period = 'year' | 'nineMonths' | 'sixMonths' | 'threeMonths' | 'month';

export interface SubscriberDataState {
  is_active: boolean;
  start_date: number | string | bigint;
  start_block: number;
  end_date: number | string | bigint;
  end_block: number;
  period: Period;
  will_renew: boolean;
  price: number | string | bigint;
}

export interface SubscriberData {
  currency_id: ActorId;
  period: Period;
  subscription_start: [number | string | bigint, number] | null;
  renewal_date: [number | string | bigint, number] | null;
}

export class Program {
  public readonly registry: TypeRegistry;
  public readonly varatube: Varatube;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
    const types: Record<string, any> = {
      Config: {
        gas_for_token_transfer: 'u64',
        gas_to_start_subscription_update: 'u64',
        block_duration: 'u32',
        min_gas_limit: 'u64',
      },
      Period: { _enum: ['Year', 'NineMonths', 'SixMonths', 'ThreeMonths', 'Month'] },
      SubscriberDataState: {
        is_active: 'bool',
        start_date: 'u64',
        start_block: 'u32',
        end_date: 'u64',
        end_block: 'u32',
        period: 'Period',
        will_renew: 'bool',
        price: 'u128',
      },
      SubscriberData: {
        currency_id: '[u8;32]',
        period: 'Period',
        subscription_start: 'Option<(u64, u32)>',
        renewal_date: 'Option<(u64, u32)>',
      },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.varatube = new Varatube(this);
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

export class Varatube {
  constructor(private _program: Program) {}

  public addTokenData(token_id: ActorId, price: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Varatube', 'AddTokenData', token_id, price],
      '(String, String, [u8;32], u128)',
      'Null',
      this._program.programId,
    );
  }

  public cancelSubscription(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Varatube', 'CancelSubscription'],
      '(String, String)',
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
      ['Varatube', 'Kill', inheritor],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public managePendingSubscription(enable: boolean): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Varatube', 'ManagePendingSubscription', enable],
      '(String, String, bool)',
      'Null',
      this._program.programId,
    );
  }

  public registerSubscription(period: Period, currency_id: ActorId, with_renewal: boolean): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Varatube', 'RegisterSubscription', period, currency_id, with_renewal],
      '(String, String, Period, [u8;32], bool)',
      'Null',
      this._program.programId,
    );
  }

  public updateConfig(
    gas_for_token_transfer: number | string | bigint | null,
    gas_to_start_subscription_update: number | string | bigint | null,
    block_duration: number | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Varatube', 'UpdateConfig', gas_for_token_transfer, gas_to_start_subscription_update, block_duration],
      '(String, String, Option<u64>, Option<u64>, Option<u32>)',
      'Null',
      this._program.programId,
    );
  }

  public updateSubscription(subscriber: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Varatube', 'UpdateSubscription', subscriber],
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
    const payload = this._program.registry.createType('(String, String)', ['Varatube', 'Admins']).toHex();
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

  public async allSubscriptions(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, SubscriberDataState]>> {
    const payload = this._program.registry.createType('(String, String)', ['Varatube', 'AllSubscriptions']).toHex();
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
      '(String, String, Vec<([u8;32], SubscriberDataState)>)',
      reply.payload,
    );
    return result[2].toJSON() as unknown as Array<[ActorId, SubscriberDataState]>;
  }

  public async config(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Config> {
    const payload = this._program.registry.createType('(String, String)', ['Varatube', 'Config']).toHex();
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

  public async currencies(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, number | string | bigint]>> {
    const payload = this._program.registry.createType('(String, String)', ['Varatube', 'Currencies']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], u128)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, number | string | bigint]>;
  }

  public async getSubscriber(
    account: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<SubscriberData | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Varatube', 'GetSubscriber', account])
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
    const result = this._program.registry.createType('(String, String, Option<SubscriberData>)', reply.payload);
    return result[2].toJSON() as unknown as SubscriberData | null;
  }

  public async subscribers(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, SubscriberData]>> {
    const payload = this._program.registry.createType('(String, String)', ['Varatube', 'Subscribers']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], SubscriberData)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, SubscriberData]>;
  }

  public subscribeToSubscriptionRegisteredEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Varatube' && getFnNamePrefix(payload) === 'SubscriptionRegistered') {
        callback(null);
      }
    });
  }

  public subscribeToSubscriptionUpdatedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Varatube' && getFnNamePrefix(payload) === 'SubscriptionUpdated') {
        callback(null);
      }
    });
  }

  public subscribeToSubscriptionCancelledEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Varatube' && getFnNamePrefix(payload) === 'SubscriptionCancelled') {
        callback(null);
      }
    });
  }

  public subscribeToPendingSubscriptionManagedEvent(
    callback: (data: null) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Varatube' && getFnNamePrefix(payload) === 'PendingSubscriptionManaged') {
        callback(null);
      }
    });
  }

  public subscribeToPaymentAddedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Varatube' && getFnNamePrefix(payload) === 'PaymentAdded') {
        callback(null);
      }
    });
  }

  public subscribeToConfigUpdatedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Varatube' && getFnNamePrefix(payload) === 'ConfigUpdated') {
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
      if (getServiceNamePrefix(payload) === 'Varatube' && getFnNamePrefix(payload) === 'Killed') {
        callback(
          this._program.registry
            .createType('(String, String, {"inheritor":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { inheritor: ActorId },
        );
      }
    });
  }
}
