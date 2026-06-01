/* eslint-disable */
/// <reference path="./global.d.ts" />

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
  public readonly dns: Dns;
  private _program?: BaseGearProgram;

  constructor(
    public api: GearApi,
    programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      ContractInfo: { admins: 'Vec<[u8;32]>', program_id: '[u8;32]', registration_time: 'String' },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);
    if (programId) {
      this._program = new BaseGearProgram(programId, api);
    }

    this.dns = new Dns(this);
  }

  public get programId(): `0x${string}` {
    if (!this._program) throw new Error(`Program ID is not set`);
    return this._program.id;
  }

  newCtorFromCode(code: Uint8Array | Buffer | HexString): TransactionBuilder<null> {
    // @ts-ignore
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      null,
      null,
      'String',
      code,
      async (programId) => {
        this._program = await BaseGearProgram.new(programId, this.api);
      },
    );
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      null,
      'New',
      null,
      null,
      'String',
      codeId,
      async (programId) => {
        this._program = await BaseGearProgram.new(programId, this.api);
      },
    );
    return builder;
  }
}

export class Dns {
  constructor(private _program: SailsProgram) {}

  public addAdminToProgram(name: string, new_admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Dns',
      'AddAdminToProgram',
      [name, new_admin],
      '(String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public addNewProgram(name: string, program_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Dns',
      'AddNewProgram',
      [name, program_id],
      '(String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public changeProgramId(name: string, new_program_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Dns',
      'ChangeProgramId',
      [name, new_program_id],
      '(String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public deleteMe(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Dns',
      'DeleteMe',
      null,
      null,
      'Null',
      this._program.programId,
    );
  }

  public deleteProgram(name: string): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Dns',
      'DeleteProgram',
      name,
      'String',
      'Null',
      this._program.programId,
    );
  }

  public removeAdminFromProgram(name: string, admin_to_remove: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Dns',
      'RemoveAdminFromProgram',
      [name, admin_to_remove],
      '(String, [u8;32])',
      'Null',
      this._program.programId,
    );
  }

  public allContracts(): QueryBuilder<Array<[string, ContractInfo]>> {
    return new QueryBuilder<Array<[string, ContractInfo]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Dns',
      'AllContracts',
      null,
      null,
      'Vec<(String, ContractInfo)>',
    );
  }

  public getAllAddresses(): QueryBuilder<Array<ActorId>> {
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Dns',
      'GetAllAddresses',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public getAllNames(): QueryBuilder<Array<string>> {
    return new QueryBuilder<Array<string>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Dns',
      'GetAllNames',
      null,
      null,
      'Vec<String>',
    );
  }

  public getContractInfoByName(name: string): QueryBuilder<ContractInfo | null> {
    return new QueryBuilder<ContractInfo | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Dns',
      'GetContractInfoByName',
      name,
      'String',
      'Option<ContractInfo>',
    );
  }

  public getNameByProgramId(program_id: ActorId): QueryBuilder<string | null> {
    return new QueryBuilder<string | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Dns',
      'GetNameByProgramId',
      program_id,
      '[u8;32]',
      'Option<String>',
    );
  }

  public subscribeToNewProgramAddedEvent(
    callback: (data: { name: string; contract_info: ContractInfo }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Dns' && getFnNamePrefix(payload) === 'NewProgramAdded') {
        callback(
          this._program.registry
            .createType('(String, String, {"name":"String","contract_info":"ContractInfo"})', message.payload)[2]
            .toJSON() as unknown as { name: string; contract_info: ContractInfo },
        );
      }
    });
  }

  public subscribeToProgramIdChangedEvent(
    callback: (data: { name: string; contract_info: ContractInfo }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Dns' && getFnNamePrefix(payload) === 'ProgramIdChanged') {
        callback(
          this._program.registry
            .createType('(String, String, {"name":"String","contract_info":"ContractInfo"})', message.payload)[2]
            .toJSON() as unknown as { name: string; contract_info: ContractInfo },
        );
      }
    });
  }

  public subscribeToProgramDeletedEvent(
    callback: (data: { name: string }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Dns' && getFnNamePrefix(payload) === 'ProgramDeleted') {
        callback(
          this._program.registry
            .createType('(String, String, {"name":"String"})', message.payload)[2]
            .toJSON() as unknown as { name: string },
        );
      }
    });
  }

  public subscribeToAdminAddedEvent(
    callback: (data: { name: string; contract_info: ContractInfo }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Dns' && getFnNamePrefix(payload) === 'AdminAdded') {
        callback(
          this._program.registry
            .createType('(String, String, {"name":"String","contract_info":"ContractInfo"})', message.payload)[2]
            .toJSON() as unknown as { name: string; contract_info: ContractInfo },
        );
      }
    });
  }

  public subscribeToAdminRemovedEvent(
    callback: (data: { name: string; contract_info: ContractInfo }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Dns' && getFnNamePrefix(payload) === 'AdminRemoved') {
        callback(
          this._program.registry
            .createType('(String, String, {"name":"String","contract_info":"ContractInfo"})', message.payload)[2]
            .toJSON() as unknown as { name: string; contract_info: ContractInfo },
        );
      }
    });
  }
}
