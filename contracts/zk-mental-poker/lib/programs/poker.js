import { BaseGearProgram, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, throwOnErrorReply, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';
export class SailsProgram {
    api;
    registry;
    poker;
    session;
    _program;
    constructor(api, programId) {
        this.api = api;
        const types = {
            GameConfig: { "admin_id": "[u8;32]", "admin_name": "String", "lobby_name": "String", "small_blind": "u128", "big_blind": "u128", "starting_bank": "u128", "time_per_move_ms": "u64" },
            SessionConfig: { "gas_to_delete_session": "u64", "minimum_session_duration_ms": "u64", "ms_per_block": "u64" },
            ZkPublicKey: { "x": "[u8; 32]", "y": "[u8; 32]", "z": "[u8; 32]" },
            SignatureInfo: { "signature_data": "SignatureData", "signature": "Option<Vec<u8>>" },
            SignatureData: { "key": "[u8;32]", "duration": "u64", "allowed_actions": "Vec<ActionsForSession>" },
            ActionsForSession: { "_enum": ["AllActions"] },
            PartialDec: { "c0": "[Vec<u8>; 3]", "delta_c0": "[Vec<u8>; 3]", "proof": "ChaumPedersenProofBytes" },
            ChaumPedersenProofBytes: { "a": "[Vec<u8>; 3]", "b": "[Vec<u8>; 3]", "z": "Vec<u8>" },
            EncryptedCard: { "c0": "[Vec<u8>; 3]", "c1": "[Vec<u8>; 3]" },
            VerificationVariables: { "proof_bytes": "ProofBytes", "public_input": "Vec<Vec<u8>>" },
            ProofBytes: { "a": "Vec<u8>", "b": "Vec<u8>", "c": "Vec<u8>" },
            Action: { "_enum": { "Fold": "Null", "Call": "Null", "Raise": { "bet": "u128" }, "Check": "Null", "AllIn": "Null" } },
            TurnManagerForActorId: { "active_ids": "Vec<[u8;32]>", "turn_index": "u64", "first_index": "u16" },
            BettingStage: { "turn": "[u8;32]", "last_active_time": "Option<u64>", "current_bet": "u128", "acted_players": "Vec<[u8;32]>" },
            Participant: { "name": "String", "balance": "u128", "pk": "ZkPublicKey" },
            Card: { "value": "u8", "suit": "Suit" },
            Suit: { "_enum": ["Spades", "Hearts", "Diamonds", "Clubs"] },
            Status: { "_enum": { "Registration": "Null", "WaitingShuffleVerification": "Null", "WaitingStart": "Null", "WaitingPartialDecryptionsForPlayersCards": "Null", "Play": { "stage": "Stage" }, "WaitingForCardsToBeDisclosed": "Null", "WaitingForAllTableCardsToBeDisclosed": "Null", "Finished": { "pots": "Vec<(u128, Vec<[u8;32]>)>" } } },
            Stage: { "_enum": ["PreFlop", "WaitingTableCardsAfterPreFlop", "Flop", "WaitingTableCardsAfterFlop", "Turn", "WaitingTableCardsAfterTurn", "River"] },
            SessionData: { "key": "[u8;32]", "expires": "u64", "allowed_actions": "Vec<ActionsForSession>", "expires_at_block": "u32" },
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
    get programId() {
        if (!this._program)
            throw new Error(`Program ID is not set`);
        return this._program.id;
    }
    newCtorFromCode(code, config, session_config, pts_actor_id, pk, session_for_admin, zk_verification_id) {
        const builder = new TransactionBuilder(this.api, this.registry, 'upload_program', undefined, 'New', [config, session_config, pts_actor_id, pk, session_for_admin, zk_verification_id], '(GameConfig, SessionConfig, [u8;32], ZkPublicKey, Option<SignatureInfo>, [u8;32])', 'String', code, async (programId) => {
            this._program = await BaseGearProgram.new(programId, this.api);
        });
        return builder;
    }
    newCtorFromCodeId(codeId, config, session_config, pts_actor_id, pk, session_for_admin, zk_verification_id) {
        const builder = new TransactionBuilder(this.api, this.registry, 'create_program', undefined, 'New', [config, session_config, pts_actor_id, pk, session_for_admin, zk_verification_id], '(GameConfig, SessionConfig, [u8;32], ZkPublicKey, Option<SignatureInfo>, [u8;32])', 'String', codeId, async (programId) => {
            this._program = await BaseGearProgram.new(programId, this.api);
        });
        return builder;
    }
}
export class Poker {
    _program;
    constructor(_program) {
        this._program = _program;
    }
    cancelGame(session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'CancelGame', session_for_account, 'Option<[u8;32]>', 'Null', this._program.programId);
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
    cancelRegistration(session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'CancelRegistration', session_for_account, 'Option<[u8;32]>', 'Null', this._program.programId);
    }
    cardDisclosure(player_decryptions, session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'CardDisclosure', [player_decryptions, session_for_account], '(Vec<PartialDec>, Option<[u8;32]>)', 'Null', this._program.programId);
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
    deletePlayer(player_id, session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'DeletePlayer', [player_id, session_for_account], '([u8;32], Option<[u8;32]>)', 'Null', this._program.programId);
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
    kill(session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'Kill', session_for_account, 'Option<[u8;32]>', 'Null', this._program.programId);
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
    register(player_name, pk, session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'Register', [player_name, pk, session_for_account], '(String, ZkPublicKey, Option<[u8;32]>)', 'Null', this._program.programId);
    }
    /**
     * Restarts the game, resetting status and refunding bets (if not Finished).
     * Panics if caller is not admin.
     * Resets game to WaitingShuffleVerification (if full) or Registration status.
     * Emits GameRestarted event with new status.
    */
    restartGame(session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'RestartGame', session_for_account, 'Option<[u8;32]>', 'Null', this._program.programId);
    }
    shuffleDeck(encrypted_deck, instances) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'ShuffleDeck', [encrypted_deck, instances], '(Vec<EncryptedCard>, Vec<VerificationVariables>)', 'Null', this._program.programId);
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
    startGame(session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'StartGame', session_for_account, 'Option<[u8;32]>', 'Null', this._program.programId);
    }
    submitPartialDecryptions(player_decryptions, session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'SubmitPartialDecryptions', [player_decryptions, session_for_account], '(Vec<PartialDec>, Option<[u8;32]>)', 'Null', this._program.programId);
    }
    submitTablePartialDecryptions(player_decryptions, session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'SubmitTablePartialDecryptions', [player_decryptions, session_for_account], '(Vec<PartialDec>, Option<[u8;32]>)', 'Null', this._program.programId);
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
    turn(action, session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Poker', 'Turn', [action, session_for_account], '(Action, Option<[u8;32]>)', 'Null', this._program.programId);
    }
    async activeParticipants(originAddress, value, atBlock) {
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
        return result[2].toJSON();
    }
    async aggPubKey(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Poker', 'AggPubKey']).toHex();
        const reply = await this._program.api.message.calculateReply({
            destination: this._program.programId,
            origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
            payload,
            value: value || 0,
            gasLimit: this._program.api.blockGasLimit.toBigInt(),
            at: atBlock,
        });
        throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
        const result = this._program.registry.createType('(String, String, ZkPublicKey)', reply.payload);
        return result[2].toJSON();
    }
    async allInPlayers(originAddress, value, atBlock) {
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
        return result[2].toJSON();
    }
    async alreadyInvestedInTheCircle(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Poker', 'AlreadyInvestedInTheCircle']).toHex();
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
        return result[2].toJSON();
    }
    async betting(originAddress, value, atBlock) {
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
        return result[2].toJSON();
    }
    async bettingBank(originAddress, value, atBlock) {
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
        return result[2].toJSON();
    }
    async config(originAddress, value, atBlock) {
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
        const result = this._program.registry.createType('(String, String, GameConfig)', reply.payload);
        return result[2].toJSON();
    }
    async encryptedTableCards(originAddress, value, atBlock) {
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
        return result[2].toJSON();
    }
    async factoryActorId(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Poker', 'FactoryActorId']).toHex();
        const reply = await this._program.api.message.calculateReply({
            destination: this._program.programId,
            origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
            payload,
            value: value || 0,
            gasLimit: this._program.api.blockGasLimit.toBigInt(),
            at: atBlock,
        });
        throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
        const result = this._program.registry.createType('(String, String, [u8;32])', reply.payload);
        return result[2].toJSON();
    }
    async participants(originAddress, value, atBlock) {
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
        return result[2].toJSON();
    }
    async playerCards(player_id, originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String, [u8;32])', ['Poker', 'PlayerCards', player_id]).toHex();
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
        return result[2].toJSON();
    }
    async ptsActorId(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Poker', 'PtsActorId']).toHex();
        const reply = await this._program.api.message.calculateReply({
            destination: this._program.programId,
            origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
            payload,
            value: value || 0,
            gasLimit: this._program.api.blockGasLimit.toBigInt(),
            at: atBlock,
        });
        throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
        const result = this._program.registry.createType('(String, String, [u8;32])', reply.payload);
        return result[2].toJSON();
    }
    async revealedPlayers(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Poker', 'RevealedPlayers']).toHex();
        const reply = await this._program.api.message.calculateReply({
            destination: this._program.programId,
            origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
            payload,
            value: value || 0,
            gasLimit: this._program.api.blockGasLimit.toBigInt(),
            at: atBlock,
        });
        throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
        const result = this._program.registry.createType('(String, String, Vec<([u8;32], (Card, Card))>)', reply.payload);
        return result[2].toJSON();
    }
    async revealedTableCards(originAddress, value, atBlock) {
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
        return result[2].toJSON();
    }
    async round(originAddress, value, atBlock) {
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
        const result = this._program.registry.createType('(String, String, u64)', reply.payload);
        return result[2].toBigInt();
    }
    async status(originAddress, value, atBlock) {
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
        return result[2].toJSON();
    }
    async tableCardsToDecrypt(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Poker', 'TableCardsToDecrypt']).toHex();
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
        return result[2].toJSON();
    }
    async waitingParticipants(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Poker', 'WaitingParticipants']).toHex();
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
        return result[2].toJSON();
    }
    subscribeToRegisteredEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'Registered') {
                callback(this._program.registry.createType('(String, String, {"participant_id":"[u8;32]","pk":"ZkPublicKey"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToPlayerDeletedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'PlayerDeleted') {
                callback(this._program.registry.createType('(String, String, {"player_id":"[u8;32]"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToRegistrationCanceledEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'RegistrationCanceled') {
                callback(this._program.registry.createType('(String, String, {"player_id":"[u8;32]"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToDeckShuffleCompleteEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'DeckShuffleComplete') {
                callback(null);
            }
        });
    }
    subscribeToGameStartedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'GameStarted') {
                callback(null);
            }
        });
    }
    subscribeToCardsDealtToPlayersEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'CardsDealtToPlayers') {
                callback(this._program.registry.createType('(String, String, Vec<([u8;32], [EncryptedCard; 2])>)', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToCardsDealtToTableEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'CardsDealtToTable') {
                callback(this._program.registry.createType('(String, String, Vec<EncryptedCard>)', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToGameRestartedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'GameRestarted') {
                callback(this._program.registry.createType('(String, String, {"status":"Status"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToSmallBlindIsSetEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'SmallBlindIsSet') {
                callback(null);
            }
        });
    }
    subscribeToBigBlindIsSetEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'BigBlindIsSet') {
                callback(null);
            }
        });
    }
    subscribeToTurnIsMadeEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'TurnIsMade') {
                callback(this._program.registry.createType('(String, String, {"action":"Action"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToNextStageEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'NextStage') {
                callback(this._program.registry.createType('(String, String, Stage)', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToFinishedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'Finished') {
                callback(this._program.registry.createType('(String, String, {"pots":"Vec<(u128, Vec<[u8;32]>)>"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToKilledEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'Killed') {
                callback(null);
            }
        });
    }
    subscribeToAllPartialDecryptionsSubmitedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'AllPartialDecryptionsSubmited') {
                callback(null);
            }
        });
    }
    subscribeToTablePartialDecryptionsSubmitedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'TablePartialDecryptionsSubmited') {
                callback(null);
            }
        });
    }
    subscribeToCardsDisclosedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'CardsDisclosed') {
                callback(null);
            }
        });
    }
    subscribeToGameCanceledEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'GameCanceled') {
                callback(null);
            }
        });
    }
    subscribeToWaitingForCardsToBeDisclosedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'WaitingForCardsToBeDisclosed') {
                callback(null);
            }
        });
    }
    subscribeToWaitingForAllTableCardsToBeDisclosedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'WaitingForAllTableCardsToBeDisclosed') {
                callback(null);
            }
        });
    }
    subscribeToRegisteredToTheNextRoundEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Poker' && getFnNamePrefix(payload) === 'RegisteredToTheNextRound') {
                callback(this._program.registry.createType('(String, String, {"participant_id":"[u8;32]","pk":"ZkPublicKey"})', message.payload)[2].toJSON());
            }
        });
    }
}
export class Session {
    _program;
    constructor(_program) {
        this._program = _program;
    }
    createSession(signature_data, signature) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Session', 'CreateSession', [signature_data, signature], '(SignatureData, Option<Vec<u8>>)', 'Null', this._program.programId);
    }
    deleteSessionFromAccount() {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Session', 'DeleteSessionFromAccount', undefined, undefined, 'Null', this._program.programId);
    }
    deleteSessionFromProgram(session_for_account) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Session', 'DeleteSessionFromProgram', session_for_account, '[u8;32]', 'Null', this._program.programId);
    }
    async sessionForTheAccount(account, originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String, [u8;32])', ['Session', 'SessionForTheAccount', account]).toHex();
        const reply = await this._program.api.message.calculateReply({
            destination: this._program.programId,
            origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
            payload,
            value: value || 0,
            gasLimit: this._program.api.blockGasLimit.toBigInt(),
            at: atBlock,
        });
        throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
        const result = this._program.registry.createType('(String, String, Option<SessionData>)', reply.payload);
        return result[2].toJSON();
    }
    async sessions(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Session', 'Sessions']).toHex();
        const reply = await this._program.api.message.calculateReply({
            destination: this._program.programId,
            origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
            payload,
            value: value || 0,
            gasLimit: this._program.api.blockGasLimit.toBigInt(),
            at: atBlock,
        });
        throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
        const result = this._program.registry.createType('(String, String, Vec<([u8;32], SessionData)>)', reply.payload);
        return result[2].toJSON();
    }
    subscribeToSessionCreatedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Session' && getFnNamePrefix(payload) === 'SessionCreated') {
                callback(null);
            }
        });
    }
    subscribeToSessionDeletedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
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
