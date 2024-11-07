import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, ActorId, ZERO_ADDRESS } from 'sails-js';

export interface PlayerInfo {
  position: number;
  balance: number;
  debt: number;
  in_jail: boolean;
  round: number | string | bigint;
  cells: `0x${string}`;
  penalty: number;
  lost: boolean;
}

export type Gear = 'bronze' | 'silver' | 'gold';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly player: Player;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
    const types: Record<string, any> = {
      PlayerInfo: {
        position: 'u8',
        balance: 'u32',
        debt: 'u32',
        in_jail: 'bool',
        round: 'u128',
        cells: 'Vec<u8>',
        penalty: 'u8',
        lost: 'bool',
      },
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
    players: Array<[ActorId, PlayerInfo]>,
    properties: Array<[ActorId, Array<Gear>, number, number] | null>,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<boolean> {
    const payload = this._program.registry
      .createType('(String, String, Vec<([u8;32], PlayerInfo)>, Vec<Option<([u8;32], Vec<Gear>, u32, u32)>>)', [
        'Player',
        'YourTurn',
        players,
        properties,
      ])
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
    const result = this._program.registry.createType('(String, String, bool)', reply.payload);
    return result[2].toJSON() as unknown as boolean;
  }
}
