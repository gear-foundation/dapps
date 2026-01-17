import { ActorId } from 'sails-js';

declare global {
  export interface GameConfig {
    admin_id: ActorId;
    admin_name: string;
    lobby_name: string;
    small_blind: number | string | bigint;
    big_blind: number | string | bigint;
    starting_bank: number | string | bigint;
    time_per_move_ms: number | string | bigint;
  }

  export interface SessionConfig {
    gas_to_delete_session: number | string | bigint;
    minimum_session_duration_ms: number | string | bigint;
    ms_per_block: number | string | bigint;
  }

  export interface ZkPublicKey {
    x: Array<number>;
    y: Array<number>;
    z: Array<number>;
  }

  export interface SignatureInfo {
    signature_data: SignatureData;
    signature: `0x${string}` | null;
  }

  export interface SignatureData {
    key: ActorId;
    duration: number | string | bigint;
    allowed_actions: Array<ActionsForSession>;
  }

  export type ActionsForSession = "AllActions";

  export interface PartialDec {
    c0: Array<`0x${string}`>;
    delta_c0: Array<`0x${string}`>;
    proof: ChaumPedersenProofBytes;
  }

  export interface ChaumPedersenProofBytes {
    a: Array<`0x${string}`>;
    b: Array<`0x${string}`>;
    z: `0x${string}`;
  }

  export interface EncryptedCard {
    c0: Array<`0x${string}`>;
    c1: Array<`0x${string}`>;
  }

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

  export type Action = 
    | { Fold: null }
    | { Call: null }
    | { Raise: { bet: number | string | bigint } }
    | { Check: null }
    | { AllIn: null };

  export interface TurnManagerForActorId {
    active_ids: Array<ActorId>;
    turn_index: number | string | bigint;
    first_index: number;
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
    pk: ZkPublicKey;
  }

  export interface Card {
    value: number;
    suit: Suit;
  }

  export type Suit = "Spades" | "Hearts" | "Diamonds" | "Clubs";

  export type Status = 
    | { Registration: null }
    | { WaitingShuffleVerification: null }
    | { WaitingStart: null }
    | { WaitingPartialDecryptionsForPlayersCards: null }
    | { Play: { stage: Stage } }
    | { WaitingForCardsToBeDisclosed: null }
    | { WaitingForAllTableCardsToBeDisclosed: null }
    | { Finished: { pots: Array<[number | string | bigint, Array<ActorId>]> } };

  export type Stage = "PreFlop" | "WaitingTableCardsAfterPreFlop" | "Flop" | "WaitingTableCardsAfterFlop" | "Turn" | "WaitingTableCardsAfterTurn" | "River";

  export interface SessionData {
    key: ActorId;
    expires: number | string | bigint;
    allowed_actions: Array<ActionsForSession>;
    expires_at_block: number;
  }
};