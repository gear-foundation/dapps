/* eslint-disable */
import { GearApi } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { ActorId, QueryBuilder, TransactionBuilder } from 'sails-js';

export interface GameInfo {
  admin_id: ActorId;
  properties_in_bank: `0x${string}`;
  players: Array<[ActorId, PlayerInfo]>;
  players_queue: Array<ActorId>;
  properties: Array<[ActorId, Array<Gear>, number, number] | null>;
  ownership: Array<ActorId>;
}

export interface PlayerInfo {
  owner_id: ActorId;
  name: string;
  position: number;
  balance: number;
  debt: number;
  in_jail: boolean;
  round: number | string | bigint;
  cells: any;
  penalty: number;
  lost: boolean;
  reservation_id: ReservationId | null;
}

export type ReservationId = [Array<number>];

export type Gear = 'Bronze' | 'Silver' | 'Gold';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly player: Player;

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      GameInfo: {
        admin_id: '[u8;32]',
        properties_in_bank: 'Vec<u8>',
        players: 'Vec<([u8;32], PlayerInfo)>',
        players_queue: 'Vec<[u8;32]>',
        properties: 'Vec<Option<([u8;32], Vec<Gear>, u32, u32)>>',
        ownership: 'Vec<[u8;32]>',
      },
      PlayerInfo: {
        owner_id: '[u8;32]',
        name: 'String',
        position: 'u8',
        balance: 'u32',
        debt: 'u32',
        in_jail: 'bool',
        round: 'u128',
        cells: 'BTreeSetForU8',
        penalty: 'u8',
        lost: 'bool',
        reservation_id: 'Option<ReservationId>',
      },
      ReservationId: '([u8; 32])',
      Gear: { _enum: ['Bronze', 'Silver', 'Gold'] },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.player = new Player(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer): TransactionBuilder<null> {
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

export class Player {
  constructor(private _program: Program) {}

  public yourTurn(game_info: GameInfo): QueryBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Player',
      'YourTurn',
      game_info,
      'GameInfo',
      'Null',
    );
  }
}
