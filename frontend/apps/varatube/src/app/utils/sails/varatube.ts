/* eslint-disable */
import { GearApi } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, QueryBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

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

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
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
    // @ts-ignore
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      [config, dns_id_and_name],
      '(Config, Option<([u8;32], String)>)',
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
      null,
      'New',
      [config, dns_id_and_name],
      '(Config, Option<([u8;32], String)>)',
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
      'Varatube',
      'AddTokenData',
      [token_id, price],
      '([u8;32], u128)',
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
      'Varatube',
      'CancelSubscription',
      null,
      null,
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
      'Varatube',
      'Kill',
      inheritor,
      '[u8;32]',
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
      'Varatube',
      'ManagePendingSubscription',
      enable,
      'bool',
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
      'Varatube',
      'RegisterSubscription',
      [period, currency_id, with_renewal],
      '(Period, [u8;32], bool)',
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
      'Varatube',
      'UpdateConfig',
      [gas_for_token_transfer, gas_to_start_subscription_update, block_duration],
      '(Option<u64>, Option<u64>, Option<u32>)',
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
      'Varatube',
      'UpdateSubscription',
      subscriber,
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
      'Varatube',
      'Admins',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public allSubscriptions(): QueryBuilder<Array<[ActorId, SubscriberDataState]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<[ActorId, SubscriberDataState]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Varatube',
      'AllSubscriptions',
      null,
      null,
      'Vec<([u8;32], SubscriberDataState)>',
    );
  }

  public config(): QueryBuilder<Config> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Config>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Varatube',
      'Config',
      null,
      null,
      'Config',
    );
  }

  public currencies(): QueryBuilder<Array<[ActorId, number | string | bigint]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<[ActorId, number | string | bigint]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Varatube',
      'Currencies',
      null,
      null,
      'Vec<([u8;32], u128)>',
    );
  }

  public getSubscriber(account: ActorId): QueryBuilder<SubscriberData | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<SubscriberData | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Varatube',
      'GetSubscriber',
      account,
      '[u8;32]',
      'Option<SubscriberData>',
    );
  }

  public subscribers(): QueryBuilder<Array<[ActorId, SubscriberData]>> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Array<[ActorId, SubscriberData]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Varatube',
      'Subscribers',
      null,
      null,
      'Vec<([u8;32], SubscriberData)>',
    );
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
