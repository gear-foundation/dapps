import { CodeId, ActorId } from 'sails-js';

declare global {
  // poker_factory
  export interface PokerFactoryConfig {
    lobby_code_id: CodeId;
    gas_for_program: number | string | bigint;
    gas_for_reply_deposit: number | string | bigint;
  }

  export interface LobbyConfig {
    admin_id: ActorId;
    admin_name: string;
    lobby_name: string;
    small_blind: number | string | bigint;
    big_blind: number | string | bigint;
    starting_bank: number | string | bigint;
    time_per_move_ms: number | string | bigint;
  }

  // poker
  export type Config = LobbyConfig;

  export interface PublicKey {
    x: Uint8Array;
    y: Uint8Array;
    z: Uint8Array;
  }

  /**
   * Serialized verifying key for zk-SNARK verification
   */
  export interface VerifyingKeyBytes {
    alpha_g1_beta_g2: `0x${string}`;
    gamma_g2_neg_pc: `0x${string}`;
    delta_g2_neg_pc: `0x${string}`;
    ic: Array<`0x${string}`>;
  }

  export interface Card {
    value: number;
    suit: Suit;
  }

  export type Suit = 'Spades' | 'Hearts' | 'Diamonds' | 'Clubs';

  /**
   * Complete verification instance containing proof and public inputs
   */
  export interface VerificationVariables {
    proof_bytes: ProofBytes;
    public_input: Array<`0x${string}`>;
  }

  /**
   * Serialized zk-SNARK proof components
   */
  export interface ProofBytes {
    a: `0x${string}`;
    b: `0x${string}`;
    c: `0x${string}`;
  }

  // ! TODO: check if this is correct
  export interface EncryptedCard {
    c0: Array<`0x${string}`>;
    c1: Array<`0x${string}`>;
  }

  export type Action =
    | { fold: null }
    | { call: null }
    | { raise: { bet: number | string | bigint } }
    | { check: null }
    | { allIn: null };

  export interface TurnManagerForActorId {
    active_ids: Array<ActorId>;
    turn_index: number | string | bigint;
  }

  export interface BettingStage {
    turn: ActorId;
    last_active_time: number | string | bigint | null;
    current_bet: number | string | bigint;
    acted_players: Array<ActorId>;
  }

  export interface Participant {
    name: string;
    balance: number | string | bigint;
    pk: PublicKey;
  }

  export type Status =
    | { registration: null }
    | { waitingShuffleVerification: null }
    | { waitingStart: null }
    | { waitingPartialDecryptionsForPlayersCards: null }
    | { play: { stage: Stage } }
    | { waitingForCardsToBeDisclosed: null }
    | { waitingForAllTableCardsToBeDisclosed: null }
    | { finished: { pots: Array<[number | string | bigint, Array<ActorId>]> } };

  export type Stage =
    | 'PreFlop'
    | 'WaitingTableCardsAfterPreFlop'
    | 'Flop'
    | 'WaitingTableCardsAfterFlop'
    | 'Turn'
    | 'WaitingTableCardsAfterTurn'
    | 'River';
}
