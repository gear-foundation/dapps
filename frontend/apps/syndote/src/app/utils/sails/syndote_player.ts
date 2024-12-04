import { ActorId, TransactionBuilder, ZERO_ADDRESS } from 'sails-js';
import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';

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

  constructor(public api: GearApi, public programId?: `0x${string}`) {
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
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      'New',
      'String',
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
      'New',
      'String',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class Player {
  constructor(private _program: Program) {}

  public async yourTurn(
    game_info: GameInfo,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<null> {
    const payload = this._program.registry
      .createType('(String, String, GameInfo)', ['Player', 'YourTurn', game_info])
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
    const result = this._program.registry.createType('(String, String, Null)', reply.payload);
    return result[2].toJSON() as unknown as null;
  }
}
