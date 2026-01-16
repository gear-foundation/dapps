import { BaseGearProgram, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, throwOnErrorReply, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';
export class PtsProgram {
    api;
    registry;
    pts;
    _program;
    constructor(api, programId) {
        this.api = api;
        const types = {};
        this.registry = new TypeRegistry();
        this.registry.setKnownTypes({ types });
        this.registry.register(types);
        if (programId) {
            this._program = new BaseGearProgram(programId, api);
        }
        this.pts = new Pts(this);
    }
    get programId() {
        if (!this._program)
            throw new Error(`Program ID is not set`);
        return this._program.id;
    }
    newCtorFromCode(code, accrual, time_ms_between_balance_receipt) {
        const builder = new TransactionBuilder(this.api, this.registry, 'upload_program', undefined, 'New', [accrual, time_ms_between_balance_receipt], '(u128, u64)', 'String', code, async (programId) => {
            this._program = await BaseGearProgram.new(programId, this.api);
        });
        return builder;
    }
    newCtorFromCodeId(codeId, accrual, time_ms_between_balance_receipt) {
        const builder = new TransactionBuilder(this.api, this.registry, 'create_program', undefined, 'New', [accrual, time_ms_between_balance_receipt], '(u128, u64)', 'String', codeId, async (programId) => {
            this._program = await BaseGearProgram.new(programId, this.api);
        });
        return builder;
    }
}
export class Pts {
    _program;
    constructor(_program) {
        this._program = _program;
    }
    addAdmin(new_admin) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Pts', 'AddAdmin', new_admin, '[u8;32]', 'Null', this._program.programId);
    }
    batchTransfer(from, to_ids, amounts) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Pts', 'BatchTransfer', [from, to_ids, amounts], '([u8;32], Vec<[u8;32]>, Vec<u128>)', 'Null', this._program.programId);
    }
    changeAccrual(new_accrual) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Pts', 'ChangeAccrual', new_accrual, 'u128', 'Null', this._program.programId);
    }
    changeTimeBetweenBalanceReceipt(new_time_between_balance_receipt) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Pts', 'ChangeTimeBetweenBalanceReceipt', new_time_between_balance_receipt, 'u64', 'Null', this._program.programId);
    }
    deleteAdmin(admin) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Pts', 'DeleteAdmin', admin, '[u8;32]', 'Null', this._program.programId);
    }
    getAccural() {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Pts', 'GetAccural', undefined, undefined, 'Null', this._program.programId);
    }
    transfer(from, to, amount) {
        if (!this._program.programId)
            throw new Error('Program ID is not set');
        return new TransactionBuilder(this._program.api, this._program.registry, 'send_message', 'Pts', 'Transfer', [from, to, amount], '([u8;32], [u8;32], u128)', 'Null', this._program.programId);
    }
    async accrual(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Pts', 'Accrual']).toHex();
        const reply = await this._program.api.message.calculateReply({
            destination: this._program.programId,
            origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
            payload,
            value: value || 0,
            gasLimit: this._program.api.blockGasLimit.toBigInt(),
            at: atBlock,
        });
        throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
        const result = this._program.registry.createType('(String, String, u128)', reply.payload);
        return result[2].toBigInt();
    }
    async admins(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Pts', 'Admins']).toHex();
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
    async getBalance(id, originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String, [u8;32])', ['Pts', 'GetBalance', id]).toHex();
        const reply = await this._program.api.message.calculateReply({
            destination: this._program.programId,
            origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
            payload,
            value: value || 0,
            gasLimit: this._program.api.blockGasLimit.toBigInt(),
            at: atBlock,
        });
        throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
        const result = this._program.registry.createType('(String, String, u128)', reply.payload);
        return result[2].toBigInt();
    }
    async getRemainingTimeMs(id, originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String, [u8;32])', ['Pts', 'GetRemainingTimeMs', id]).toHex();
        const reply = await this._program.api.message.calculateReply({
            destination: this._program.programId,
            origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
            payload,
            value: value || 0,
            gasLimit: this._program.api.blockGasLimit.toBigInt(),
            at: atBlock,
        });
        throwOnErrorReply(reply.code, reply.payload.toU8a(), this._program.api.specVersion, this._program.registry);
        const result = this._program.registry.createType('(String, String, Option<u64>)', reply.payload);
        return result[2].toJSON();
    }
    async timeMsBetweenBalanceReceipt(originAddress, value, atBlock) {
        const payload = this._program.registry.createType('(String, String)', ['Pts', 'TimeMsBetweenBalanceReceipt']).toHex();
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
    subscribeToNewAdminAddedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'NewAdminAdded') {
                callback(this._program.registry.createType('(String, String, [u8;32])', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToAdminDeletedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'AdminDeleted') {
                callback(this._program.registry.createType('(String, String, [u8;32])', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToAccrualChangedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'AccrualChanged') {
                callback(this._program.registry.createType('(String, String, u128)', message.payload)[2].toBigInt());
            }
        });
    }
    subscribeToTimeBetweenBalanceReceiptChangedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'TimeBetweenBalanceReceiptChanged') {
                callback(this._program.registry.createType('(String, String, u64)', message.payload)[2].toBigInt());
            }
        });
    }
    subscribeToAccrualReceivedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'AccrualReceived') {
                callback(this._program.registry.createType('(String, String, {"id":"[u8;32]","accrual":"u128"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToSubtractionIsDoneEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'SubtractionIsDone') {
                callback(this._program.registry.createType('(String, String, {"id":"[u8;32]","amount":"u128"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToAdditionIsDoneEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'AdditionIsDone') {
                callback(this._program.registry.createType('(String, String, {"id":"[u8;32]","amount":"u128"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToTransferedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'Transfered') {
                callback(this._program.registry.createType('(String, String, {"from":"[u8;32]","to":"[u8;32]","amount":"u128"})', message.payload)[2].toJSON());
            }
        });
    }
    subscribeToBatchTransferedEvent(callback) {
        return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {
            ;
            if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
                return;
            }
            const payload = message.payload.toHex();
            if (getServiceNamePrefix(payload) === 'Pts' && getFnNamePrefix(payload) === 'BatchTransfered') {
                callback(this._program.registry.createType('(String, String, {"from":"[u8;32]","to_ids":"Vec<[u8;32]>","amounts":"Vec<u128>"})', message.payload)[2].toJSON());
            }
        });
    }
}
