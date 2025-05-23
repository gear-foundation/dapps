/* eslint-disable */

import { GearApi, HexString, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import {
  TransactionBuilder,
  ActorId,
  throwOnErrorReply,
  getServiceNamePrefix,
  getFnNamePrefix,
  ZERO_ADDRESS,
} from 'sails-js';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly poker: Poker;

  constructor(
    public api: GearApi,
    private _programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      Config: {
        admin_id: '[u8;32]',
        admin_name: 'String',
        lobby_name: 'String',
        small_blind: 'u128',
        big_blind: 'u128',
        number_of_participants: 'u16',
        starting_bank: 'u128',
      },
      PublicKey: { x: '[u8; 32]', y: '[u8; 32]', z: '[u8; 32]' },
      VerifyingKeyBytes: {
        alpha_g1_beta_g2: 'Vec<u8>',
        gamma_g2_neg_pc: 'Vec<u8>',
        delta_g2_neg_pc: 'Vec<u8>',
        ic: 'Vec<Vec<u8>>',
      },
      Card: { value: 'u8', suit: 'Suit' },
      Suit: { _enum: ['Spades', 'Hearts', 'Diamonds', 'Clubs'] },
      EncryptedCard: { c0: '[Vec<u8>; 3]', c1: '[Vec<u8>; 3]' },
      VerificationVariables: { proof_bytes: 'ProofBytes', public_input: 'Vec<Vec<u8>>' },
      ProofBytes: { a: 'Vec<u8>', b: 'Vec<u8>', c: 'Vec<u8>' },
      Action: { _enum: { Fold: 'Null', Call: 'Null', Raise: { bet: 'u128' }, Check: 'Null', AllIn: 'Null' } },
      TurnManagerForActorId: { active_ids: 'Vec<[u8;32]>', turn_index: 'u64' },
      BettingStage: {
        turn: '[u8;32]',
        last_active_time: 'Option<u64>',
        current_bet: 'u128',
        acted_players: 'Vec<[u8;32]>',
      },
      Participant: { name: 'String', balance: 'u128', card_1: 'Option<u32>', card_2: 'Option<u32>', pk: 'PublicKey' },
      Status: {
        _enum: {
          Registration: 'Null',
          WaitingShuffleVerification: 'Null',
          WaitingStart: 'Null',
          WaitingPartialDecryptionsForPlayersCards: 'Null',
          Play: { stage: 'Stage' },
          WaitingForCardsToBeDisclosed: 'Null',
          Finished: { winners: 'Vec<[u8;32]>', cash_prize: 'Vec<u128>' },
        },
      },
      Stage: {
        _enum: [
          'PreFlop',
          'WaitingTableCardsAfterPreFlop',
          'Flop',
          'WaitingTableCardsAfterFlop',
          'Turn',
          'WaitingTableCardsAfterTurn',
          'River',
        ],
      },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.poker = new Poker(this);
  }

  public get programId(): `0x${string}` {
    if (!this._programId) throw new Error(`Program ID is not set`);
    return this._programId;
  }

  newCtorFromCode(
    code: Uint8Array | Buffer | HexString,
    config: Config,
    pts_actor_id: ActorId,
    pk: PublicKey,
    vk_shuffle_bytes: VerifyingKeyBytes,
    vk_decrypt_bytes: VerifyingKeyBytes,
  ): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', config, pts_actor_id, pk, vk_shuffle_bytes, vk_decrypt_bytes],
      '(String, Config, [u8;32], PublicKey, VerifyingKeyBytes, VerifyingKeyBytes)',
      'String',
      code,
    );

    this._programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(
    codeId: `0x${string}`,
    config: Config,
    pts_actor_id: ActorId,
    pk: PublicKey,
    vk_shuffle_bytes: VerifyingKeyBytes,
    vk_decrypt_bytes: VerifyingKeyBytes,
  ) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', config, pts_actor_id, pk, vk_shuffle_bytes, vk_decrypt_bytes],
      '(String, Config, [u8;32], PublicKey, VerifyingKeyBytes, VerifyingKeyBytes)',
      'String',
      codeId,
    );

    this._programId = builder.programId;
    return builder;
  }
}

export class Poker {
  constructor(private _program: Program) {}

  public cancelGame(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Poker', 'CancelGame'],
      '(String, String)',
      'Null',
      this._program.programId,
    );
  }

  public cardDisclosure(
    id_to_cards: Array<[ActorId, [Card, Card]]>,
    table_cards: Array<Card>,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Poker', 'CardDisclosure', id_to_cards, table_cards],
      '(String, String, Vec<([u8;32], (Card, Card))>, Vec<Card>)',
      'Null',
      this._program.programId,
    );
  }

  public register(player_name: string, pk: PublicKey): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Poker', 'Register', player_name, pk],
      '(String, String, String, PublicKey)',
      'Null',
      this._program.programId,
    );
  }

  public shuffleDeck(
    encrypted_deck: Array<EncryptedCard>,
    instances: Array<VerificationVariables>,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Poker', 'ShuffleDeck', encrypted_deck, instances],
      '(String, String, Vec<EncryptedCard>, Vec<VerificationVariables>)',
      'Null',
      this._program.programId,
    );
  }

  public startGame(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Poker', 'StartGame'],
      '(String, String)',
      'Null',
      this._program.programId,
    );
  }

  public submitAllPartialDecryptions(
    cards_by_player: Array<[ActorId, Array<EncryptedCard>]>,
    proofs: Array<VerificationVariables>,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Poker', 'SubmitAllPartialDecryptions', cards_by_player, proofs],
      '(String, String, Vec<([u8;32], [EncryptedCard; 2])>, Vec<VerificationVariables>)',
      'Null',
      this._program.programId,
    );
  }

  public submitRevealedTableCards(
    new_cards: Array<Card>,
    proofs: Array<VerificationVariables>,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Poker', 'SubmitRevealedTableCards', new_cards, proofs],
      '(String, String, Vec<Card>, Vec<VerificationVariables>)',
      'Null',
      this._program.programId,
    );
  }

  public submitTablePartialDecryptions(
    decryptions: Array<[EncryptedCard, Array<`0x${string}`>]>,
    proofs: Array<VerificationVariables>,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Poker', 'SubmitTablePartialDecryptions', decryptions, proofs],
      '(String, String, Vec<(EncryptedCard, [Vec<u8>; 3])>, Vec<VerificationVariables>)',
      'Null',
      this._program.programId,
    );
  }

  public turn(action: Action): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['Poker', 'Turn', action],
      '(String, String, Action)',
      'Null',
      this._program.programId,
    );
  }

  public async activeParticipants(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<TurnManagerForActorId> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'ActiveParticipants']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, TurnManagerForActorId)', reply.payload);
    return result[2].toJSON() as unknown as TurnManagerForActorId;
  }

  public async allInPlayers(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<ActorId>> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'AllInPlayers']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<[u8;32]>)', reply.payload);
    return result[2].toJSON() as unknown as Array<ActorId>;
  }

  public async alreadyInvestedInTheCircle(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, number | string | bigint]>> {
    const payload = this._program.registry
      .createType('(String, String)', ['Poker', 'AlreadyInvestedInTheCircle'])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], u128)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, number | string | bigint]>;
  }

  public async betting(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<BettingStage | null> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'Betting']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Option<BettingStage>)', reply.payload);
    return result[2].toJSON() as unknown as BettingStage | null;
  }

  public async bettingBank(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, number | string | bigint]>> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'BettingBank']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], u128)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, number | string | bigint]>;
  }

  public async config(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Config> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'Config']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Config)', reply.payload);
    return result[2].toJSON() as unknown as Config;
  }

  public async encryptedTableCards(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<EncryptedCard>> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'EncryptedTableCards']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<EncryptedCard>)', reply.payload);
    return result[2].toJSON() as unknown as Array<EncryptedCard>;
  }

  public async participants(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<[ActorId, Participant]>> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'Participants']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<([u8;32], Participant)>)', reply.payload);
    return result[2].toJSON() as unknown as Array<[ActorId, Participant]>;
  }

  public async playerCards(
    player_id: ActorId,
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<EncryptedCard> | null> {
    const payload = this._program.registry
      .createType('(String, String, [u8;32])', ['Poker', 'PlayerCards', player_id])
      .toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Option<[EncryptedCard; 2]>)', reply.payload);
    return result[2].toJSON() as unknown as Array<EncryptedCard> | null;
  }

  public async revealedTableCards(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Array<Card>> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'RevealedTableCards']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Vec<Card>)', reply.payload);
    return result[2].toJSON() as unknown as Array<Card>;
  }

  public async round(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<number> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'Round']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, u32)', reply.payload);
    return result[2].toNumber() as unknown as number;
  }

  public async status(
    originAddress?: string,
    value?: number | string | bigint,
    atBlock?: `0x${string}`,
  ): Promise<Status> {
    const payload = this._program.registry.createType('(String, String)', ['Poker', 'Status']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock,
    });
    throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
    const result = this._program.registry.createType('(String, String, Status)', reply.payload);
    return result[2].toJSON() as unknown as Status;
  }

  public subscribeToRegisteredEvent(
    callback: (data: { participant_id: ActorId; pk: PublicKey; all_registered: boolean }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'Registered') {
        callback(
          this._program.registry
            .createType(
              '(String, String, {"participant_id":"[u8;32]","pk":"PublicKey","all_registered":"bool"})',
              message.payload,
            )[2]
            .toJSON() as unknown as { participant_id: ActorId; pk: PublicKey; all_registered: boolean },
        );
      }
    });
  }

  public subscribeToGameStartedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'GameStarted') {
        callback(null);
      }
    });
  }

  public subscribeToCardsDealtToPlayersEvent(
    callback: (data: Array<[ActorId, Array<EncryptedCard>]>) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'CardsDealtToPlayers') {
        callback(
          this._program.registry
            .createType('(String, String, Vec<([u8;32], [EncryptedCard; 2])>)', message.payload)[2]
            .toJSON() as unknown as Array<[ActorId, Array<EncryptedCard>]>,
        );
      }
    });
  }

  public subscribeToCardsDealtToTableEvent(
    callback: (data: Array<EncryptedCard>) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'CardsDealtToTable') {
        callback(
          this._program.registry
            .createType('(String, String, Vec<EncryptedCard>)', message.payload)[2]
            .toJSON() as unknown as Array<EncryptedCard>,
        );
      }
    });
  }

  public subscribeToGameCanceledEvent(
    callback: (data: { status: Status }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'GameCanceled') {
        callback(
          this._program.registry
            .createType('(String, String, {"status":"Status"})', message.payload)[2]
            .toJSON() as unknown as { status: Status },
        );
      }
    });
  }

  public subscribeToSmallBlindIsSetEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'SmallBlindIsSet') {
        callback(null);
      }
    });
  }

  public subscribeToBigBlindIsSetEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'BigBlindIsSet') {
        callback(null);
      }
    });
  }

  public subscribeToTurnIsMadeEvent(callback: (data: { action: Action }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'TurnIsMade') {
        callback(
          this._program.registry
            .createType('(String, String, {"action":"Action"})', message.payload)[2]
            .toJSON() as unknown as { action: Action },
        );
      }
    });
  }

  public subscribeToNextStageEvent(callback: (data: Stage) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'NextStage') {
        callback(
          this._program.registry.createType('(String, String, Stage)', message.payload)[2].toJSON() as unknown as Stage,
        );
      }
    });
  }

  public subscribeToFinishedEvent(
    callback: (data: { winners: Array<ActorId>; cash_prize: Array<number | string | bigint> }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'Finished') {
        callback(
          this._program.registry
            .createType('(String, String, {"winners":"Vec<[u8;32]>","cash_prize":"Vec<u128>"})', message.payload)[2]
            .toJSON() as unknown as { winners: Array<ActorId>; cash_prize: Array<number | string | bigint> },
        );
      }
    });
  }
}
