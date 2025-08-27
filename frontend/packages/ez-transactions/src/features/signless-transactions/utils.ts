import { decodeAddress, GearKeyring, IGearEvent, IGearVoucherEvent } from '@gear-js/api';
import { Account, AlertContainerFactory } from '@gear-js/react-hooks';
import { SubmittableExtrinsic } from '@polkadot/api/types';
import { encodeAddress } from '@polkadot/keyring';
import { KeyringPair$Json, KeyringPair } from '@polkadot/keyring/types';
import { Codec } from '@polkadot/types/types';

type Options = Partial<{
  onSuccess: () => void;
  onError: (error: Error) => void;
  onFinally: () => void;
  pair?: KeyringPair;
}>;

const MULTIPLIER = {
  MS: 1000,
  SECONDS: 60,
  MINUTES: 60,
  HOURS: 24,
};

export async function sendTransaction<E extends keyof IGearEvent | keyof IGearVoucherEvent | 'Transfer'>(
  submitted: SubmittableExtrinsic<'promise'>,
  account: KeyringPair,
  methods: E[],
  { onSuccess = () => {}, onError = () => {}, onFinally = () => {} }: Options = {},
  /* eslint-disable  @typescript-eslint/no-explicit-any */
): Promise<any[]> {
  /* eslint-disable  @typescript-eslint/no-explicit-any */
  const result = new Array(methods.length) as Codec[];
  return new Promise((resolve, reject) => {
    submitted
      .signAndSend(account, ({ events, status }) => {
        events.forEach(({ event }) => {
          const { method, data } = event;
          if (methods.includes(method as E) && status.isInBlock) {
            result[methods.indexOf(method as E)] = data;
          } else if (method === 'ExtrinsicFailed') {
            onError(new Error('ExtrinsicFailed'));
            onFinally();
            reject(new Error(data.toString()));
          }
        });
        if (status.isInBlock) {
          onSuccess();
          resolve([...result, status.asInBlock.toHex()]);
        }
      })
      .catch((error) => {
        const errorMessage = error instanceof Error ? error.message : String(error);
        console.log(error);
        onError(new Error(errorMessage));
        onFinally();
        reject(new Error(errorMessage));
      });
  });
}

const getVaraAddress = (value: string) => {
  const VARA_SS58_FORMAT = 137;
  const decodedAddress = decodeAddress(value);

  return encodeAddress(decodedAddress, VARA_SS58_FORMAT);
};

const getMilliseconds = (minutes: number) => minutes * MULTIPLIER.MS * MULTIPLIER.SECONDS;

const getDoubleDigits = (value: number) => (value < 10 ? `0${value}` : value);

const getMinutesFromSeconds = (seconds: number) => seconds / MULTIPLIER.SECONDS;

const getDHMS = (ms: number) => {
  const seconds = Math.floor((ms / MULTIPLIER.MS) % MULTIPLIER.SECONDS);
  const minutes = Math.floor((ms / (MULTIPLIER.MS * MULTIPLIER.SECONDS)) % MULTIPLIER.MINUTES);
  const hours = Math.floor((ms / (MULTIPLIER.MS * MULTIPLIER.SECONDS * MULTIPLIER.MINUTES)) % MULTIPLIER.HOURS);
  const days = Math.floor(ms / (MULTIPLIER.MS * MULTIPLIER.SECONDS * MULTIPLIER.MINUTES * MULTIPLIER.HOURS));

  return `${days ? `${days} days, ` : ''}${getDoubleDigits(hours)}:${getDoubleDigits(minutes)}:${getDoubleDigits(
    seconds,
  )}`;
};

const shortenString = (str: string, length: number): string => `${str.slice(0, length)}...${str.slice(-length)}`;

const copyToClipboard = ({
  alert,
  value,
  successfulText,
}: {
  alert?: AlertContainerFactory;
  value: string;
  successfulText?: string;
}) => {
  const onSuccess = () => {
    if (alert) {
      alert.success(successfulText || 'Copied');
    }
  };
  const onError = () => {
    if (alert) {
      alert.error('Copy error');
    }
  };

  function unsecuredCopyToClipboard(text: string) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    try {
      document.execCommand('copy');
      onSuccess();
    } catch (err) {
      console.error('Unable to copy to clipboard', err);
      onError();
    }
    document.body.removeChild(textArea);
  }

  if (window.isSecureContext && navigator.clipboard) {
    navigator.clipboard
      .writeText(value)
      .then(() => onSuccess())
      .catch(() => onError());
  } else {
    unsecuredCopyToClipboard(value);
  }
};

const getUnlockedPair = (pair: KeyringPair$Json, password: string) => GearKeyring.fromJson(pair, password);

const signHex = async (_account: Account, hexToSign: `0x${string}`) => {
  const { signer } = _account;
  const { signRaw } = signer;

  if (!signRaw) {
    throw new Error('signRaw is not a function');
  }

  return signRaw({ address: _account.address, data: hexToSign, type: 'bytes' });
};

export {
  getMilliseconds,
  getMinutesFromSeconds,
  getDHMS,
  getVaraAddress,
  shortenString,
  copyToClipboard,
  getUnlockedPair,
  signHex,
};
