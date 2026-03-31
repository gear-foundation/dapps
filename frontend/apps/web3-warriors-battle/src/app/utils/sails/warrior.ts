/* eslint-disable */
import { GearApi } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, QueryBuilder, ActorId } from 'sails-js';

export interface Appearance {
  head_index: number;
  hat_index: number;
  body_index: number;
  accessory_index: number;
  body_color: string;
  back_color: string;
}

export class WarriorProgram {
  public readonly registry: TypeRegistry;
  public readonly warrior: Warrior;

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      Appearance: {
        head_index: 'u16',
        hat_index: 'u16',
        body_index: 'u16',
        accessory_index: 'u16',
        body_color: 'String',
        back_color: 'String',
      },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.warrior = new Warrior(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer): TransactionBuilder<null> {
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
    );

    this.programId = builder.programId;
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
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class Warrior {
  constructor(private _program: WarriorProgram) {}

  public getAppearance(): QueryBuilder<Appearance> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<Appearance>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Warrior',
      'GetAppearance',
      null,
      null,
      'Appearance',
    );
  }

  public getOwner(): QueryBuilder<ActorId> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<ActorId>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Warrior',
      'GetOwner',
      null,
      null,
      '[u8;32]',
    );
  }
}
