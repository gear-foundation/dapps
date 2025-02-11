import { GearApi, HexString, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

type ActorId = HexString;

export interface ProgramState {
  streams: Array<[string, Stream]>;
  users: Array<[ActorId, Profile]>;
  admins: Array<ActorId>;
  dns_info: [ActorId, string] | null;
}

export interface Stream {
  broadcaster: ActorId;
  start_time: number | string | bigint;
  end_time: number | string | bigint;
  title: string;
  img_link: string;
  description: string | null;
}

export interface Profile {
  name: string | null;
  surname: string | null;
  img_link: string | null;
  time_zone: string | null;
  stream_ids: Array<string>;
  subscribers: Array<ActorId>;
  subscriptions: Array<Subscription>;
}

export interface Subscription {
  account_id: ActorId;
  sub_date: number | string | bigint;
}

export class Program {
  public readonly registry: TypeRegistry;
  public readonly w3Bstreaming: W3Bstreaming;

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      ProgramState: {
        streams: 'Vec<(String, Stream)>',
        users: 'Vec<([u8;32], Profile)>',
        admins: 'Vec<[u8;32]>',
        dns_info: 'Option<([u8;32], String)>',
      },
      Stream: {
        broadcaster: '[u8;32]',
        start_time: 'u64',
        end_time: 'u64',
        title: 'String',
        img_link: 'String',
        description: 'Option<String>',
      },
      Profile: {
        name: 'Option<String>',
        surname: 'Option<String>',
        img_link: 'Option<String>',
        time_zone: 'Option<String>',
        stream_ids: 'Vec<String>',
        subscribers: 'Vec<[u8;32]>',
        subscriptions: 'Vec<Subscription>',
      },
      Subscription: { account_id: '[u8;32]', sub_date: 'u64' },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.w3Bstreaming = new W3Bstreaming(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, dns_id_and_name: [ActorId, string] | null): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', dns_id_and_name],
      '(String, Option<([u8;32], String)>)',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, dns_id_and_name: [ActorId, string] | null) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', dns_id_and_name],
      '(String, Option<([u8;32], String)>)',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class W3Bstreaming {
  constructor(private _program: Program) {}

  public addAdmin(new_admin_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['W3Bstreaming', 'AddAdmin', new_admin_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public deleteStream(stream_id: string): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['W3Bstreaming', 'DeleteStream', stream_id],
      '(String, String, String)',
      'Null',
      this._program.programId,
    );
  }

  public editProfile(
    name: string | null,
    surname: string | null,
    img_link: string | null,
    time_zone: string | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['W3Bstreaming', 'EditProfile', name, surname, img_link, time_zone],
      '(String, String, Option<String>, Option<String>, Option<String>, Option<String>)',
      'Null',
      this._program.programId,
    );
  }

  public editStream(
    stream_id: string,
    start_time: number | string | bigint | null,
    end_time: number | string | bigint | null,
    title: string | null,
    img_link: string | null,
    description: string | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['W3Bstreaming', 'EditStream', stream_id, start_time, end_time, title, img_link, description],
      '(String, String, String, Option<u64>, Option<u64>, Option<String>, Option<String>, Option<String>)',
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
      ['W3Bstreaming', 'Kill', inheritor],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public newStream(
    title: string,
    description: string | null,
    start_time: number | string | bigint,
    end_time: number | string | bigint,
    img_link: string,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['W3Bstreaming', 'NewStream', title, description, start_time, end_time, img_link],
      '(String, String, String, Option<String>, u64, u64, String)',
      'Null',
      this._program.programId,
    );
  }

  public subscribe(account_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['W3Bstreaming', 'Subscribe', account_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public unsubscribe(account_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['W3Bstreaming', 'Unsubscribe', account_id],
      '(String, String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public async getState(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<ProgramState> {
    const payload = this._program.registry.createType('(String, String)', ['W3Bstreaming', 'GetState']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId!,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, ProgramState)', reply.payload);
    return result[2].toJSON() as unknown as ProgramState;
  }

  public subscribeToStreamIsScheduledEvent(
    callback: (data: { id: string }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'W3Bstreaming' && getFnNamePrefix(payload) === 'StreamIsScheduled') {
        callback(
          this._program.registry
            .createType('(String, String, {"id":"String"})', message.payload)[2]
            .toJSON() as unknown as { id: string },
        );
      }
    });
  }

  public subscribeToStreamDeletedEvent(callback: (data: { id: string }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'W3Bstreaming' && getFnNamePrefix(payload) === 'StreamDeleted') {
        callback(
          this._program.registry
            .createType('(String, String, {"id":"String"})', message.payload)[2]
            .toJSON() as unknown as { id: string },
        );
      }
    });
  }

  public subscribeToStreamEditedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'W3Bstreaming' && getFnNamePrefix(payload) === 'StreamEdited') {
        callback(null);
      }
    });
  }

  public subscribeToSubscribedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'W3Bstreaming' && getFnNamePrefix(payload) === 'Subscribed') {
        callback(null);
      }
    });
  }

  public subscribeToUnsubscribedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'W3Bstreaming' && getFnNamePrefix(payload) === 'Unsubscribed') {
        callback(null);
      }
    });
  }

  public subscribeToProfileEditedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'W3Bstreaming' && getFnNamePrefix(payload) === 'ProfileEdited') {
        callback(null);
      }
    });
  }

  public subscribeToAdminAddedEvent(
    callback: (data: { new_admin_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'W3Bstreaming' && getFnNamePrefix(payload) === 'AdminAdded') {
        callback(
          this._program.registry
            .createType('(String, String, {"new_admin_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { new_admin_id: ActorId },
        );
      }
    });
  }

  public subscribeToKilledEvent(callback: (data: { inheritor: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'W3Bstreaming' && getFnNamePrefix(payload) === 'Killed') {
        callback(
          this._program.registry
            .createType('(String, String, {"inheritor":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { inheritor: ActorId },
        );
      }
    });
  }
}
