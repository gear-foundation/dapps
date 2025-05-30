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
    number_of_participants: number;
    starting_bank: number | string | bigint;
  }

  // poker
  export interface Config {
    admin_id: ActorId;
    admin_name: string;
    lobby_name: string;
    small_blind: number | string | bigint;
    big_blind: number | string | bigint;
    number_of_participants: number;
    starting_bank: number | string | bigint;
  }

  export interface PublicKey {
    x: Uint8Array;
    y: Uint8Array;
    z: Uint8Array;
  }

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

  export type Suit = 'spades' | 'hearts' | 'diamonds' | 'clubs';

  export interface EncryptedCard {
    c0: Array<`0x${string}`>;
    c1: Array<`0x${string}`>;
  }

  export interface VerificationVariables {
    proof_bytes: ProofBytes;
    public_input: Array<`0x${string}`>;
  }

  export interface ProofBytes {
    a: `0x${string}`;
    b: `0x${string}`;
    c: `0x${string}`;
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
    card_1: number | null;
    card_2: number | null;
    pk: PublicKey;
  }

  export type Status =
    | { registration: null }
    | { waitingShuffleVerification: null }
    | { waitingStart: null }
    | { waitingPartialDecryptionsForPlayersCards: null }
    | { play: { stage: Stage } }
    | { waitingForCardsToBeDisclosed: null }
    | { finished: { winners: Array<ActorId>; cash_prize: Array<number | string | bigint> } };

  export type Stage =
    | 'preFlop'
    | 'waitingTableCardsAfterPreFlop'
    | 'flop'
    | 'waitingTableCardsAfterFlop'
    | 'turn'
    | 'waitingTableCardsAfterTurn'
    | 'river';
}
