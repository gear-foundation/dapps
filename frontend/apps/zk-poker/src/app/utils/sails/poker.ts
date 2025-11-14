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
  public readonly poker: Poker;
  public readonly session: Session;
  private _program?: BaseGearProgram;

  constructor(
    public api: GearApi,
    programId?: `0x${string}`,
  ) {
    const types: Record<string, any> = {
      GameConfig: {
        admin_id: '[u8;32]',
        admin_name: 'String',
        lobby_name: 'String',
        small_blind: 'u128',
        big_blind: 'u128',
        starting_bank: 'u128',
        time_per_move_ms: 'u64',
      },
      SessionConfig: { gas_to_delete_session: 'u64', minimum_session_duration_ms: 'u64', ms_per_block: 'u64' },
      ZkPublicKey: { x: '[u8; 32]', y: '[u8; 32]', z: '[u8; 32]' },
      SignatureInfo: { signature_data: 'SignatureData', signature: 'Option<Vec<u8>>' },
      SignatureData: { key: '[u8;32]', duration: 'u64', allowed_actions: 'Vec<ActionsForSession>' },
      ActionsForSession: { _enum: ['AllActions'] },
      PartialDec: { c0: '[Vec<u8>; 3]', delta_c0: '[Vec<u8>; 3]', proof: 'ChaumPedersenProofBytes' },
      ChaumPedersenProofBytes: { a: '[Vec<u8>; 3]', b: '[Vec<u8>; 3]', z: 'Vec<u8>' },
      EncryptedCard: { c0: '[Vec<u8>; 3]', c1: '[Vec<u8>; 3]' },
      VerificationVariables: { proof_bytes: 'ProofBytes', public_input: 'Vec<Vec<u8>>' },
      ProofBytes: { a: 'Vec<u8>', b: 'Vec<u8>', c: 'Vec<u8>' },
      Action: { _enum: { Fold: 'Null', Call: 'Null', Raise: { bet: 'u128' }, Check: 'Null', AllIn: 'Null' } },
      TurnManagerForActorId: { active_ids: 'Vec<[u8;32]>', turn_index: 'u64', first_index: 'u16' },
      BettingStage: {
        turn: '[u8;32]',
        last_active_time: 'Option<u64>',
        current_bet: 'u128',
        acted_players: 'Vec<[u8;32]>',
      },
      Participant: { name: 'String', balance: 'u128', pk: 'ZkPublicKey' },
      Card: { value: 'u8', suit: 'Suit' },
      Suit: { _enum: ['Spades', 'Hearts', 'Diamonds', 'Clubs'] },
      Status: {
        _enum: {
          Registration: 'Null',
          WaitingShuffleVerification: 'Null',
          WaitingStart: 'Null',
          WaitingPartialDecryptionsForPlayersCards: 'Null',
          Play: { stage: 'Stage' },
          WaitingForCardsToBeDisclosed: 'Null',
          WaitingForAllTableCardsToBeDisclosed: 'Null',
          Finished: { pots: 'Vec<(u128, Vec<[u8;32]>)>' },
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
      SessionData: {
        key: '[u8;32]',
        expires: 'u64',
        allowed_actions: 'Vec<ActionsForSession>',
        expires_at_block: 'u32',
      },
    };

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);
    if (programId) {
      this._program = new BaseGearProgram(programId, api);
    }

    this.poker = new Poker(this);
    this.session = new Session(this);
  }

  public get programId(): `0x${string}` {
    if (!this._program) throw new Error(`Program ID is not set`);
    return this._program.id;
  }

  newCtorFromCode(
    code: Uint8Array | Buffer | HexString,
    config: GameConfig,
    session_config: SessionConfig,
    pts_actor_id: ActorId,
    pk: ZkPublicKey,
    session_for_admin: SignatureInfo | null,
    zk_verification_id: ActorId,
  ): TransactionBuilder<null> {
    // @ts-ignore
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      null,
      'New',
      [config, session_config, pts_actor_id, pk, session_for_admin, zk_verification_id],
      '(GameConfig, SessionConfig, [u8;32], ZkPublicKey, Option<SignatureInfo>, [u8;32])',
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
    config: GameConfig,
    session_config: SessionConfig,
    pts_actor_id: ActorId,
    pk: ZkPublicKey,
    session_for_admin: SignatureInfo | null,
    zk_verification_id: ActorId,
  ) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      null,
      'New',
      [config, session_config, pts_actor_id, pk, session_for_admin, zk_verification_id],
      '(GameConfig, SessionConfig, [u8;32], ZkPublicKey, Option<SignatureInfo>, [u8;32])',
      'String',
      codeId,
      async (programId) => {
        this._program = await BaseGearProgram.new(programId, this.api);
      },
    );
    return builder;
  }
}

export class Poker {
  constructor(private _program: SailsProgram) {}

  public cancelGame(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'CancelGame',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Cancels player registration and refunds their balance via PTS contract.
   *
   * Panics if:
   * - current status is invalid for cancellation;
   * - caller is not a registered player.
   *
   * Sends a transfer request to PTS contract to return points to the player.
   * Removes player data and emits `RegistrationCanceled` event on success.
   */
  public cancelRegistration(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'CancelRegistration',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  public cardDisclosure(
    player_decryptions: Array<PartialDec>,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'CardDisclosure',
      [player_decryptions, session_for_account],
      '(Vec<PartialDec>, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Admin-only function to forcibly remove a player and refund their balance.
   *
   * Panics if:
   * - caller is not admin or tries to delete themselves
   * - wrong game status (not Registration/WaitingShuffleVerification)
   * - player doesn't exist
   *
   * Performs:
   * 1. Transfers player's balance back to user via PTS contract
   * 2. Removes player from all participant lists
   * 3. Resets status to Registration
   * 4. Emits PlayerDeleted event
   */
  public deletePlayer(player_id: ActorId, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'DeletePlayer',
      [player_id, session_for_account],
      '([u8;32], Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Admin-only function to terminate the lobby and refund all players.
   *
   * Panics if:
   * - caller is not admin
   * - wrong game status (not Registration/WaitingShuffleVerification/Finished/WaitingStart)
   *
   * Performs:
   * 1. Batch transfer of all player balances via PTS contract
   * 2. Sends DeleteLobby request to PokerFactory
   * 3. Emits Killed event and transfers remaining funds to admin
   *
   * WARNING: Irreversible operation
   */
  public kill(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'Kill',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Registers a player by sending a transfer request to the PTS contract (starting_bank points).
   *
   * Panics if:
   * - status is not `Registration`;
   * - player is already registered.
   *
   * Sends a message to the PTS contract (pts_actor_id) to transfer points to this contract.
   * On success, updates participant data and emits a `Registered` event.
   */
  public register(player_name: string, pk: ZkPublicKey, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'Register',
      [player_name, pk, session_for_account],
      '(String, ZkPublicKey, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Restarts the game, resetting status and refunding bets (if not Finished).
   * Panics if caller is not admin.
   * Resets game to WaitingShuffleVerification (if full) or Registration status.
   * Emits GameRestarted event with new status.
   */
  public restartGame(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'RestartGame',
      session_for_account,
      'Option<[u8;32]>',
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
      'Poker',
      'ShuffleDeck',
      [encrypted_deck, instances],
      '(Vec<EncryptedCard>, Vec<VerificationVariables>)',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Admin-only function to start the poker game after setup.
   *
   * Panics if:
   * - caller is not admin
   * - wrong status (not WaitingStart)
   *
   * Performs:
   * 1. Processes small/big blinds (handles all-in cases)
   * 2. Initializes betting stage
   * 3. Updates game status and emits GameStarted event
   *
   * Note: Handles edge cases where players can't cover blinds
   */
  public startGame(session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'StartGame',
      session_for_account,
      'Option<[u8;32]>',
      'Null',
      this._program.programId,
    );
  }

  public submitPartialDecryptions(
    player_decryptions: Array<PartialDec>,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'SubmitPartialDecryptions',
      [player_decryptions, session_for_account],
      '(Vec<PartialDec>, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public submitTablePartialDecryptions(
    player_decryptions: Array<PartialDec>,
    session_for_account: ActorId | null,
  ): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'SubmitTablePartialDecryptions',
      [player_decryptions, session_for_account],
      '(Vec<PartialDec>, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  /**
   * Processes player actions during betting rounds.
   *
   * Panics if:
   * - Wrong game status
   * - Not player's turn
   * - Invalid action (e.g. check when bet exists)
   *
   * Handles:
   * - Fold/Call/Check/Raise/AllIn actions
   * - Turn timers and skips
   * - Game end conditions (single player left)
   * - Stage transitions
   *
   * Emits TurnIsMade and NextStage events
   */
  public turn(action: Action, session_for_account: ActorId | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Poker',
      'Turn',
      [action, session_for_account],
      '(Action, Option<[u8;32]>)',
      'Null',
      this._program.programId,
    );
  }

  public activeParticipants(): QueryBuilder<TurnManagerForActorId> {
    return new QueryBuilder<TurnManagerForActorId>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'ActiveParticipants',
      null,
      null,
      'TurnManagerForActorId',
    );
  }

  public aggPubKey(): QueryBuilder<ZkPublicKey> {
    return new QueryBuilder<ZkPublicKey>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'AggPubKey',
      null,
      null,
      'ZkPublicKey',
    );
  }

  public allInPlayers(): QueryBuilder<Array<ActorId>> {
    return new QueryBuilder<Array<ActorId>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'AllInPlayers',
      null,
      null,
      'Vec<[u8;32]>',
    );
  }

  public alreadyInvestedInTheCircle(): QueryBuilder<Array<[ActorId, number | string | bigint]>> {
    return new QueryBuilder<Array<[ActorId, number | string | bigint]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'AlreadyInvestedInTheCircle',
      null,
      null,
      'Vec<([u8;32], u128)>',
    );
  }

  public betting(): QueryBuilder<BettingStage | null> {
    return new QueryBuilder<BettingStage | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'Betting',
      null,
      null,
      'Option<BettingStage>',
    );
  }

  public bettingBank(): QueryBuilder<Array<[ActorId, number | string | bigint]>> {
    return new QueryBuilder<Array<[ActorId, number | string | bigint]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'BettingBank',
      null,
      null,
      'Vec<([u8;32], u128)>',
    );
  }

  public config(): QueryBuilder<GameConfig> {
    return new QueryBuilder<GameConfig>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'Config',
      null,
      null,
      'GameConfig',
    );
  }

  public encryptedCards(player_id: ActorId): QueryBuilder<Array<EncryptedCard> | null> {
    return new QueryBuilder<Array<EncryptedCard> | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'EncryptedCards',
      player_id,
      '[u8;32]',
      'Option<[EncryptedCard; 2]>',
    );
  }

  public encryptedTableCards(): QueryBuilder<Array<EncryptedCard>> {
    return new QueryBuilder<Array<EncryptedCard>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'EncryptedTableCards',
      null,
      null,
      'Vec<EncryptedCard>',
    );
  }

  public factoryActorId(): QueryBuilder<ActorId> {
    return new QueryBuilder<ActorId>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'FactoryActorId',
      null,
      null,
      '[u8;32]',
    );
  }

  public participants(): QueryBuilder<Array<[ActorId, Participant]>> {
    return new QueryBuilder<Array<[ActorId, Participant]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'Participants',
      null,
      null,
      'Vec<([u8;32], Participant)>',
    );
  }

  public playerCards(player_id: ActorId): QueryBuilder<Array<EncryptedCard> | null> {
    return new QueryBuilder<Array<EncryptedCard> | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'PlayerCards',
      player_id,
      '[u8;32]',
      'Option<[EncryptedCard; 2]>',
    );
  }

  public ptsActorId(): QueryBuilder<ActorId> {
    return new QueryBuilder<ActorId>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'PtsActorId',
      null,
      null,
      '[u8;32]',
    );
  }

  public revealedPlayers(): QueryBuilder<Array<[ActorId, [Card, Card]]>> {
    return new QueryBuilder<Array<[ActorId, [Card, Card]]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'RevealedPlayers',
      null,
      null,
      'Vec<([u8;32], (Card, Card))>',
    );
  }

  public revealedTableCards(): QueryBuilder<Array<Card>> {
    return new QueryBuilder<Array<Card>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'RevealedTableCards',
      null,
      null,
      'Vec<Card>',
    );
  }

  public round(): QueryBuilder<bigint> {
    return new QueryBuilder<bigint>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'Round',
      null,
      null,
      'u64',
    );
  }

  public status(): QueryBuilder<Status> {
    return new QueryBuilder<Status>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'Status',
      null,
      null,
      'Status',
    );
  }

  public tableCardsToDecrypt(): QueryBuilder<Array<EncryptedCard>> {
    return new QueryBuilder<Array<EncryptedCard>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'TableCardsToDecrypt',
      null,
      null,
      'Vec<EncryptedCard>',
    );
  }

  public waitingParticipants(): QueryBuilder<Array<[ActorId, Participant]>> {
    return new QueryBuilder<Array<[ActorId, Participant]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Poker',
      'WaitingParticipants',
      null,
      null,
      'Vec<([u8;32], Participant)>',
    );
  }

  public subscribeToRegisteredEvent(
    callback: (data: { participant_id: ActorId; pk: ZkPublicKey }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'Registered') {
        callback(
          this._program.registry
            .createType('(String, String, {"participant_id":"[u8;32]","pk":"ZkPublicKey"})', message.payload)[2]
            .toJSON() as unknown as { participant_id: ActorId; pk: ZkPublicKey },
        );
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
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'PlayerDeleted') {
        callback(
          this._program.registry
            .createType('(String, String, {"player_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { player_id: ActorId },
        );
      }
    });
  }

  public subscribeToRegistrationCanceledEvent(
    callback: (data: { player_id: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'RegistrationCanceled') {
        callback(
          this._program.registry
            .createType('(String, String, {"player_id":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { player_id: ActorId },
        );
      }
    });
  }

  public subscribeToDeckShuffleCompleteEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'DeckShuffleComplete') {
        callback(null);
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

  public subscribeToGameRestartedEvent(
    callback: (data: { status: Status }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'GameRestarted') {
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
    callback: (data: { pots: Array<[number | string | bigint, Array<ActorId>]> }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'Finished') {
        callback(
          this._program.registry
            .createType('(String, String, {"pots":"Vec<(u128, Vec<[u8;32]>)>"})', message.payload)[2]
            .toJSON() as unknown as { pots: Array<[number | string | bigint, Array<ActorId>]> },
        );
      }
    });
  }

  public subscribeToKilledEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'Killed') {
        callback(null);
      }
    });
  }

  public subscribeToAllPartialDecryptionsSubmitedEvent(
    callback: (data: null) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'AllPartialDecryptionsSubmited') {
        callback(null);
      }
    });
  }

  public subscribeToTablePartialDecryptionsSubmitedEvent(
    callback: (data: null) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'TablePartialDecryptionsSubmited') {
        callback(null);
      }
    });
  }

  public subscribeToCardsDisclosedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'CardsDisclosed') {
        callback(null);
      }
    });
  }

  public subscribeToGameCanceledEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'GameCanceled') {
        callback(null);
      }
    });
  }

  public subscribeToWaitingForCardsToBeDisclosedEvent(
    callback: (data: null) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'WaitingForCardsToBeDisclosed') {
        callback(null);
      }
    });
  }

  public subscribeToWaitingForAllTableCardsToBeDisclosedEvent(
    callback: (data: null) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (
        getServiceNamePrefix(payload) === 'Poker' &&
        getFnNamePrefix(payload) === 'WaitingForAllTableCardsToBeDisclosed'
      ) {
        callback(null);
      }
    });
  }

  public subscribeToRegisteredToTheNextRoundEvent(
    callback: (data: { participant_id: ActorId; pk: ZkPublicKey }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'RegisteredToTheNextRound') {
        callback(
          this._program.registry
            .createType('(String, String, {"participant_id":"[u8;32]","pk":"ZkPublicKey"})', message.payload)[2]
            .toJSON() as unknown as { participant_id: ActorId; pk: ZkPublicKey },
        );
      }
    });
  }

  public subscribeToAdminChangedEvent(
    callback: (data: { old_admin: ActorId; new_admin: ActorId }) => void | Promise<void>,
  ): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'AdminChanged') {
        callback(
          this._program.registry
            .createType('(String, String, {"old_admin":"[u8;32]","new_admin":"[u8;32]"})', message.payload)[2]
            .toJSON() as unknown as { old_admin: ActorId; new_admin: ActorId },
        );
      }
    });
  }
}

export class Session {
  constructor(private _program: SailsProgram) {}

  public createSession(signature_data: SignatureData, signature: `0x${string}` | null): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Session',
      'CreateSession',
      [signature_data, signature],
      '(SignatureData, Option<Vec<u8>>)',
      'Null',
      this._program.programId,
    );
  }

  public deleteSessionFromAccount(): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Session',
      'DeleteSessionFromAccount',
      null,
      null,
      'Null',
      this._program.programId,
    );
  }

  public deleteSessionFromProgram(session_for_account: ActorId): TransactionBuilder<null> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<null>(
      this._program.api,
      this._program.registry,
      'send_message',
      'Session',
      'DeleteSessionFromProgram',
      session_for_account,
      '[u8;32]',
      'Null',
      this._program.programId,
    );
  }

  public sessionForTheAccount(account: ActorId): QueryBuilder<SessionData | null> {
    return new QueryBuilder<SessionData | null>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Session',
      'SessionForTheAccount',
      account,
      '[u8;32]',
      'Option<SessionData>',
    );
  }

  public sessions(): QueryBuilder<Array<[ActorId, SessionData]>> {
    return new QueryBuilder<Array<[ActorId, SessionData]>>(
      this._program.api,
      this._program.registry,
      this._program.programId,
      'Session',
      'Sessions',
      null,
      null,
      'Vec<([u8;32], SessionData)>',
    );
  }

  public subscribeToSessionCreatedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Session' && getFnNamePrefix(payload) === 'SessionCreated') {
        callback(null);
      }
    });
  }

  public subscribeToSessionDeletedEvent(callback: (data: null) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'Session' && getFnNamePrefix(payload) === 'SessionDeleted') {
        callback(null);
      }
    });
  }
}
