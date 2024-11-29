import { GearApi, HexString, VoucherIssuedData, IUpdateVoucherParams } from '@gear-js/api';
import { waitReady } from '@polkadot/wasm-crypto';
import { hexToU8a, } from '@polkadot/util';

import { Keyring } from '@polkadot/api';

const secondsToBlock = 3;

export class GaslessService  {
  private api: GearApi;
  private readonly voucherAccount;

  constructor() {
    this.api = new GearApi({ providerAddress: process.env.NODE_URL });
    this.voucherAccount = this.getVoucherAccount();
  }

  /**
   * Issues a voucher for the given account, programId, amount, and duration.
   *
   * @param account - The account to issue the voucher for.
   * @param programId - The programId to issue the voucher for.
   * @param amount - The amount to issue the voucher for.
   * @param durationInSec - The duration to issue the voucher for.
   * @returns The voucherId.
   */
  async issue(account: HexString, programId: HexString, amount: number, durationInSec: number): Promise<string> {
    await Promise.all([this.api.isReadyOrError, waitReady()])

    const durationInBlocks = Math.round(durationInSec / secondsToBlock);

    const { extrinsic } = await this.api.voucher.issue(account, amount * 1e12, durationInBlocks, [programId]);

    const voucherId = await new Promise<HexString>((resolve, reject) => {
      extrinsic.signAndSend(this.voucherAccount, ({
        events,
        status,
      }) => {
        if (status.isInBlock) {
          const viEvent = events.find(({ event }) => event.method === 'VoucherIssued');
          if (viEvent) {
            const data = viEvent.event.data as VoucherIssuedData;
            resolve(data.voucherId.toHex());
          } else {
            const efEvent = events.find(({ event }) => event.method === 'ExtrinsicFailed');

            reject(efEvent ? this.api.getExtrinsicFailedError(efEvent?.event) : 'VoucherIssued event not found');
          }
        }
      });
    });

    return voucherId;
  }

  /**
   * Prolongs the voucher with the given voucherId, account, balance, and prolongDurationInSec.
   *
   * @param voucherId - The voucherId to prolong
   * @param account - The account to prolong the voucher for
   * @param balance - The required balance to top up the voucher
   * @param prolongDurationInSec - The duration to prolong the voucher for
   */
  async prolong(voucherId: HexString, account: string, balance: number, prolongDurationInSec: number) {
    const voucherBalance = (await this.api.balance.findOut(voucherId)).toBigInt() / BigInt(1e12);
    const durationInBlocks = Math.round(prolongDurationInSec / secondsToBlock);

    const topUp = BigInt(balance) - voucherBalance;

    const params: IUpdateVoucherParams = {};

    if (prolongDurationInSec) {
      params.prolongDuration = durationInBlocks;
    }

    if (topUp > 0) {
      params.balanceTopUp = topUp * BigInt(1e12);
    }

    const tx = this.api.voucher.update(account, voucherId, params);

    await new Promise<void>((resolve, reject) => {
      tx.signAndSend(this.voucherAccount, ({
        events,
        status,
      }) => {
        if (status.isInBlock) {
          const vuEvent = events.find(({ event }) => event.method === 'VoucherUpdated');
          if (vuEvent) {
            resolve();
          } else {
            const efEvent = events.find(({ event }) => event.method === 'ExtrinsicFailed');
            if (efEvent) {
              reject(JSON.stringify(this.api.getExtrinsicFailedError(efEvent?.event)));
            } else {
              reject(new Error('VoucherUpdated event not found'));
            }
          }
        }
      });
    });
  }

  /**
   * Revokes the voucher with the given voucherId and account.
   *
   * @param voucherId - The voucherId to revoke
   * @param account - The account to revoke the voucher for
   */
  async revoke(voucherId: HexString, account: string) {
    const tx = this.api.voucher.revoke(account, voucherId);
    await new Promise<void>((resolve, reject) => {
      tx.signAndSend(this.voucherAccount, ({
        events,
        status,
      }) => {
        if (status.isInBlock) {
          const vuEvent = events.find(({ event }) => event.method === 'VoucherRevoked');
          if (vuEvent) {
            resolve();
          } else {
            const efEvent = events.find(({ event }) => event.method === 'ExtrinsicFailed');
            if (efEvent) {
              reject(JSON.stringify(this.api.getExtrinsicFailedError(efEvent?.event)));
            } else {
              reject(new Error('VoucherRevoked event not found'));
            }
          }
        }
      });
    });
  }

  private getVoucherAccount() {
    const seed = process.env.VOUCHER_ACCOUNT;
    const keyring = new Keyring({
      type: 'sr25519',
      ss58Format: 137,
    });
    const voucherAccount = keyring.addFromSeed(hexToU8a(seed));
    return voucherAccount;
  }
}
