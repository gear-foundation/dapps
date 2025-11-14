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
  public readonly pts: Pts;
  private _program?: BaseGearProgram;

  constructor(
    public api: GearApi,
    programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {};

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);
    if (programId) {
      this._program = new BaseGearProgram(programId, api);
    }

    this.pts = new Pts(this);
  }

  public get programId(): `0x${string}` {
    if (!this._program) throw new Error(`Program ID is not set`);
    return this._program.id;
  }

  newCtorFromCode(
    code: Uint8Array | Buffer | HexString,
    accrual: number | string | bigint,
    time_ms_between_balance_receipt: number | string | bigint,
  ): TransactionBuilder<null> {
    // @ts-ignore
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      [accrual, time_ms_between_balance_receipt],
      '(u128, u64)',
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
    accrual: number | string | bigint,
    time_ms_between_balance_receipt: number | string | bigint,
  ) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      null,
      'New',
      [accrual, time_ms_between_balance_receipt],
      '(u128, u64)',
      'String',
      codeId,
      async (programId) => {
        this._program = await BaseGearProgram.new(programId, this.api);
      },
    );
    return builder;
  }
}

export class Pts {
  constructor(private _program: SailsProgram) {}

  public addAdmin(new_admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Pts',
      'AddAdmin',
      new_admin,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public changeAccrual(new_accrual: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Pts',
      'ChangeAccrual',
      new_accrual,
      'u128',
      'Null',
      this._program.programId,
    );
  }

  public changeTimeBetweenBalanceReceipt(
    new_time_between_balance_receipt: number | string | bigint,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Pts',
      'ChangeTimeBetweenBalanceReceipt',
      new_time_between_balance_receipt,
      'u64',
      'Null',
      this._program.programId,
    );
  }

  public deleteAdmin(admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Pts',
      'DeleteAdmin',
      admin,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public getAccural(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Pts',
      'GetAccural',
      null,
      null,
      'Null',
      this._program.programId,
    );
  }

  public transfer($from: ActorId, to: ActorId, amount: number | string | bigint): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Pts',
      'Transfer',
      [$from, to, amount],
      '([u8;32], [u8;32], u128)',
      'Null',
      this._program.programId,
    );
  }

  public accrual(): QueryBuilder<bigint> {
    return new QueryBuilder<bigint>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Pts',
      'Accrual',
      null,
      null,
      'u128',
    );
  }

  public admins(): QueryBuilder<Array<ActorId>> {
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Pts',
      'Admins',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public getBalance(id: ActorId): QueryBuilder<bigint> {
    return new QueryBuilder<bigint>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Pts',
      'GetBalance',
      id,
      '[u8;32]',
      'u128',
    );
  }

  public getRemainingTimeMs(id: ActorId): QueryBuilder<number | string | bigint | null> {
    return new QueryBuilder<number | string | bigint | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Pts',
      'GetRemainingTimeMs',
      id,
      '[u8;32]',
      'Option<u64>',
    );
  }

  public timeMsBetweenBalanceReceipt(): QueryBuilder<bigint> {
    return new QueryBuilder<bigint>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Pts',
      'TimeMsBetweenBalanceReceipt',
      null,
      null,
      'u64',
    );
  }

  public subscribeToNewAdminAddedEvent(callback: (data: ActorId) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'NewAdminAdded') {
        callback(
          this._program.registry
            .createType('(String, String, [u8;32])', message.payload)[2]
            .toJSON() as unknown as ActorId,
        );
      }
    });
  }

  public subscribeToAdminDeletedEvent(callback: (data: ActorId) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'AdminDeleted') {
        callback(
          this._program.registry
            .createType('(String, String, [u8;32])', message.payload)[2]
            .toJSON() as unknown as ActorId,
        );
      }
    });
  }

  public subscribeToAccrualChangedEvent(callback: (data: bigint) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'AccrualChanged') {
        callback(
          this._program.registry
            .createType('(String, String, u128)', message.payload)[2]
            .toBigInt() as unknown as bigint,
        );
      }
    });
  }

  public subscribeToTimeBetweenBalanceReceiptChangedEvent(
    callback: (data: bigint) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'TimeBetweenBalanceReceiptChanged') {
        callback(
          this._program.registry
            .createType('(String, String, u64)', message.payload)[2]
            .toBigInt() as unknown as bigint,
        );
      }
    });
  }

  public subscribeToAccrualReceivedEvent(
    callback: (data: { id: ActorId; accrual: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'AccrualReceived') {
        callback(
          this._program.registry
            .createType('(String, String, {"id":"[u8;32]","accrual":"u128"})', message.payload)[2]
            .toJSON() as unknown as { id: ActorId; accrual: number | string | bigint },
        );
      }
    });
  }

  public subscribeToSubtractionIsDoneEvent(
    callback: (data: { id: ActorId; amount: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'SubtractionIsDone') {
        callback(
          this._program.registry
            .createType('(String, String, {"id":"[u8;32]","amount":"u128"})', message.payload)[2]
            .toJSON() as unknown as { id: ActorId; amount: number | string | bigint },
        );
      }
    });
  }

  public subscribeToAdditionIsDoneEvent(
    callback: (data: { id: ActorId; amount: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'AdditionIsDone') {
        callback(
          this._program.registry
            .createType('(String, String, {"id":"[u8;32]","amount":"u128"})', message.payload)[2]
            .toJSON() as unknown as { id: ActorId; amount: number | string | bigint },
        );
      }
    });
  }

  public subscribeToTransferedEvent(
    callback: (data: { from: ActorId; to: ActorId; amount: number | string | bigint }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'Transfered') {
        callback(
          this._program.registry
            .createType('(String, String, {"from":"[u8;32]","to":"[u8;32]","amount":"u128"})', message.payload)[2]
            .toJSON() as unknown as { from: ActorId; to: ActorId; amount: number | string | bigint },
        );
      }
    });
  }
}
