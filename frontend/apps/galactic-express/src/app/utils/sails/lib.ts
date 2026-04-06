/* eslint-disable @typescript-eslint/no-explicit-any, @typescript-eslint/no-floating-promises, @typescript-eslint/ban-ts-comment */
import { GearApi, HexString } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, QueryBuilder, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

type ActorId = HexString;

export interface Participant {
  id: ActorId;
  name: string;
  fuel_amount: number;
  payload_amount: number;
}

export interface State {
  games: Array<[ActorId, GameState]>;
  player_to_game_id: Array<[ActorId, ActorId]>;
  dns_info: [ActorId, string] | null;
  admin: ActorId;
}

export interface GameState {
  admin: ActorId;
  admin_name: string;
  altitude: number;
  weather: Weather;
  reward: number | string | bigint;
  stage: StageState;
  bid: number | string | bigint;
}

export type Weather = 'clear' | 'cloudy' | 'rainy' | 'stormy' | 'thunder' | 'tornado';

export type StageState = { registration: Array<[ActorId, Participant]> } | { results: Results };

export interface Results {
  turns: Array<Array<[ActorId, Turn]>>;
  rankings: Array<[ActorId, number | string | bigint]>;
  participants: Array<[ActorId, Participant]>;
}

export type Turn = { alive: { fuel_left: number; payload_amount: number } } | { destroyed: HaltReason };

export type HaltReason =
  | 'payloadOverload'
  | 'fuelOverload'
  | 'separationFailure'
  | 'asteroidCollision'
  | 'fuelShortage'
  | 'engineFailure';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly galacticExpress: GalacticExpress;

  constructor(
    public api: GearApi,
    public programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      Participant: { id: '[u8;32]', name: 'String', fuel_amount: 'u8', payload_amount: 'u8' },
      State: {
        games: 'Vec<([u8;32], GameState)>',
        player_to_game_id: 'Vec<([u8;32], [u8;32])>',
        dns_info: 'Option<([u8;32], String)>',
        admin: '[u8;32]',
      },
      GameState: {
        admin: '[u8;32]',
        admin_name: 'String',
        altitude: 'u16',
        weather: 'Weather',
        reward: 'u128',
        stage: 'StageState',
        bid: 'u128',
      },
      Weather: { _enum: ['Clear', 'Cloudy', 'Rainy', 'Stormy', 'Thunder', 'Tornado'] },
      StageState: { _enum: { Registration: 'Vec<([u8;32], Participant)>', Results: 'Results' } },
      Results: {
        turns: 'Vec<Vec<([u8;32], Turn)>>',
        rankings: 'Vec<([u8;32], u128)>',
        participants: 'Vec<([u8;32], Participant)>',
      },
      Turn: { _enum: { Alive: { fuel_left: 'u8', payload_amount: 'u8' }, Destroyed: 'HaltReason' } },
      HaltReason: {
        _enum: [
          'PayloadOverload',
          'FuelOverload',
          'SeparationFailure',
          'AsteroidCollision',
          'FuelShortage',
          'EngineFailure',
        ],
      },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.galacticExpress = new GalacticExpress(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, dns_id_and_name: [ActorId, string] | null): TransactionBuilder<null> {
    // @ts-ignore
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      dns_id_and_name,
      'Option<([u8;32], String)>',
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
      null,
      'New',
      dns_id_and_name,
      'Option<([u8;32], String)>',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class GalacticExpress {
  constructor(private _program: Program) {}

  public cancelGame(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'GalacticExpress',
      'CancelGame',
      null,
      null,
      'Null',
      this._program.programId,
    );
  }

  public cancelRegister(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'GalacticExpress',
      'CancelRegister',
      null,
      null,
      'Null',
      this._program.programId,
    );
  }

  public changeAdmin(new_admin: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'GalacticExpress',
      'ChangeAdmin',
      new_admin,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public createNewSession(name: string): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'GalacticExpress',
      'CreateNewSession',
      name,
      'String',
      'Null',
      this._program.programId,
    );
  }

  public deletePlayer(player_id: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'GalacticExpress',
      'DeletePlayer',
      player_id,
      '[u8;32]',
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
      'GalacticExpress',
      'Kill',
      inheritor,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public leaveGame(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'GalacticExpress',
      'LeaveGame',
      null,
      null,
      'Null',
      this._program.programId,
    );
  }

  public register(creator: ActorId, participant: Participant): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'GalacticExpress',
      'Register',
      [creator, participant],
      '([u8;32], Participant)',
      'Null',
      this._program.programId,
    );
  }

  public startGame(fuel_amount: number, payload_amount: number): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'GalacticExpress',
      'StartGame',
      [fuel_amount, payload_amount],
      '(u8, u8)',
      'Null',
      this._program.programId,
    );
  }

  public admin(): QueryBuilder<ActorId> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<ActorId>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'GalacticExpress',
      'Admin',
      null,
      null,
      '[u8;32]',
    );
  }

  public all(): QueryBuilder<State> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<State>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'GalacticExpress',
      'All',
      null,
      null,
      'State',
    );
  }

  public dnsInfo(): QueryBuilder<[ActorId, string] | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<[ActorId, string] | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'GalacticExpress',
      'DnsInfo',
      null,
      null,
      'Option<([u8;32], String)>',
    );
  }

  public getGame(player_id: ActorId): QueryBuilder<GameState | null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new QueryBuilder<GameState | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'GalacticExpress',
      'GetGame',
      player_id,
      '[u8;32]',
      'Option<GameState>',
    );
  }

  public subscribeToGameFinishedEvent(callback: (data: Results) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'GalacticExpress' && getFnNamePrefix(payload) === 'GameFinished') {
        callback(
          this._program.registry
            .createType('(String, String, Results)', message.payload)[2]
            .toJSON() as unknown as Results,
        );
      }
    });
  }

  public subscribeToNewSessionCreatedEvent(
    callback: (data: {
      altitude: number;
      weather: Weather;
      reward: number | string | bigint;
      bid: number | string | bigint;
    }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'GalacticExpress' && getFnNamePrefix(payload) === 'NewSessionCreated') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"altitude":"u16","weather":"Weather","reward":"u128","bid":"u128"})',
              message.payload,
            )[2]
            .toJSON() as unknown as {
            altitude: number;
            weather: Weather;
            reward: number | string | bigint;
            bid: number | string | bigint;
          },
        );
      }
    });
  }

  public subscribeToRegisteredEvent(
    callback: (data: [ActorId, Participant]) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'GalacticExpress' && getFnNamePrefix(payload) === 'Registered') {
        callback(
          this._program.registry
            .createType('(String, String, ([u8;32], Participant))', message.payload)[2]
            .toJSON() as unknown as [ActorId, Participant],
        );
      }
    });
  }

  public subscribeToRegistrationCanceledEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'GalacticExpress' && getFnNamePrefix(payload) === 'RegistrationCanceled') {
        callback(null);
      }
    });
  }

  public subscribeToPlayerDeletedEvent(
    callback: (data: { player_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'GalacticExpress' && getFnNamePrefix(payload) === 'PlayerDeleted') {
        callback(
          this._program.registry
            .createType('(String, String, {"player_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { player_id: ActorId },
        );
      }
    });
  }

  public subscribeToGameCanceledEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'GalacticExpress' && getFnNamePrefix(payload) === 'GameCanceled') {
        callback(null);
      }
    });
  }

  public subscribeToGameLeftEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'GalacticExpress' && getFnNamePrefix(payload) === 'GameLeft') {
        callback(null);
      }
    });
  }

  public subscribeToAdminChangedEvent(
    callback: (data: { new_admin: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'GalacticExpress' && getFnNamePrefix(payload) === 'AdminChanged') {
        callback(
          this._program.registry
            .createType('(String, String, {"new_admin":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { new_admin: ActorId },
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
      if (getServiceNamePrefix(payload) === 'GalacticExpress' && getFnNamePrefix(payload) === 'Killed') {
        callback(
          this._program.registry
            .createType('(String, String, {"inheritor":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { inheritor: ActorId },
        );
      }
    });
  }
}
