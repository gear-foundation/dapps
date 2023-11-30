import { Keyring } from '@polkadot/api';
import { mnemonicGenerate } from '@polkadot/util-crypto';

const MULTIPLIER = {
  MS: 1000,
  SECONDS: 60,
  MINUTES: 60,
  HOURS: 24,
};

const getRandomPair = (password: string) => {
  const seed = mnemonicGenerate();

  const keyring = new Keyring({ type: 'sr25519' });
  const pair = keyring.addFromMnemonic(seed);

  return pair.toJson(password);
};

const getMilliseconds = (minutes: number) => minutes * MULTIPLIER.MS * MULTIPLIER.SECONDS;

const getDoubleDigits = (value: number) => (value < 10 ? `0${value}` : value);

const getHMS = (ms: number) => {
  const seconds = Math.floor((ms / MULTIPLIER.MS) % MULTIPLIER.SECONDS);
  const minutes = Math.floor((ms / (MULTIPLIER.MS * MULTIPLIER.SECONDS)) % MULTIPLIER.MINUTES);
  const hours = Math.floor((ms / (MULTIPLIER.MS * MULTIPLIER.SECONDS * MULTIPLIER.MINUTES)) % MULTIPLIER.HOURS);

  return `${getDoubleDigits(hours)}:${getDoubleDigits(minutes)}:${getDoubleDigits(seconds)}`;
};

export { getRandomPair, getMilliseconds, getHMS };
