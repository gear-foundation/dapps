import { decodeAddress } from '@gear-js/api';
import { Keyring } from '@polkadot/api';
import { mnemonicGenerate } from '@polkadot/util-crypto';

const getRandomAccount = () => {
  const privateKey = mnemonicGenerate();

  const keyring = new Keyring({ type: 'sr25519' });

  const pair = keyring.addFromMnemonic(privateKey);
  const publicKey = decodeAddress(pair.address);

  return { publicKey, privateKey };
};

const getMilliseconds = (minutes: number) => {
  const SECONDS_MULTIPLIER = 60;
  const MILLISECONDS_MULTIPLIER = 1000;

  return minutes * SECONDS_MULTIPLIER * MILLISECONDS_MULTIPLIER;
};

export { getRandomAccount, getMilliseconds };
