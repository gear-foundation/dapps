import { GearApi } from '@gear-js/api';
import { Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { hexToU8a, isHex } from '@polkadot/util';

export function getAccounts(accounts: Record<string, string>) {
  const keyring = new Keyring({ type: 'sr25519' });
  const result: Record<string, KeyringPair> = {};
  Object.entries(accounts).forEach(([name, seed]) => {
    if (seed.startsWith('//')) {
      result[name] = keyring.addFromUri(seed);
    } else if (isHex(seed)) {
      result[name] = keyring.addFromSeed(hexToU8a(seed));
    } else {
      result[name] = keyring.addFromMnemonic(seed);
    }
  });

  return result;
}

export async function fundAccounts(
  api: GearApi,
  accounts: Record<string, KeyringPair>,
  prefunded: string,
  fundAccounts: string[],
) {
  const txs = fundAccounts.map((acc) => api.balance.transfer(accounts[acc].address, 1_000_000_000_000_000));
  return new Promise((resolve) => {
    api.tx.utility.batchAll(txs).signAndSend(accounts[prefunded], (result) => {
      result.status.isFinalized && resolve('ok');
    });
  });
}
