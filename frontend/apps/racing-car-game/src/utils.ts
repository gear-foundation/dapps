import { ProgramMetadata } from '@gear-js/api';
import { SignlessTransactionsProviderProps } from '@dapps-frontend/signless-transactions';
import { AlertContainerFactory } from '@gear-js/react-hooks/dist/esm/types';
import { Bytes } from '@polkadot/types';
import { Codec } from '@polkadot/types/types';
import clsx from 'clsx';
import { useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { DecodedReply } from './types';

export const cx = (...styles: string[]) => clsx(...styles);

export const copyToClipboard = async ({
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

export const get = <T>(url: string) =>
  fetch(url, {
    method: 'GET',
  }).then(async (res) => {
    const json = await res.json();
    return json as T;
  });

export function ScrollToTop() {
  const { pathname } = useLocation();

  useEffect(() => {
    document.documentElement.scrollTo({
      top: 0,
      left: 0,
    });
  }, [pathname]);

  return null;
}

export const withoutCommas = (value: string) => (typeof value === 'string' ? value.replace(/,/g, '') : value);

export const isMobileDevice = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
  navigator.userAgent,
);

export const logger = (message: unknown | unknown[]) => {
  const date = new Date();
  let milliseconds = '';
  const milli = date.getMilliseconds();

  if (milli < 10) {
    milliseconds = `00${milli}`;
  } else if (milli < 100) {
    milliseconds = `0${milli}`;
  } else {
    milliseconds = `${milli}`;
  }

  const time = `${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}.${milliseconds}`;

  console.log(time, message);
};

/**
 * Get first element of tuple type
 *  */
export const createSignatureType: SignlessTransactionsProviderProps['createSignatureType'] = (
  metadata,
  payloadToSign,
) => {
  if (!metadata.types?.others?.output) {
    throw new Error(`Metadata type doesn't exist`);
  }

  const data = metadata.createType(metadata.types.others.output, [payloadToSign]) as unknown as Codec[];
  return data[0].toHex();
};

const getDecodedPayload = (payload: Bytes, meta?: ProgramMetadata) => {
  if (meta?.types.others.output) {
    return meta.createType(meta?.types.others.output, [null, payload]).toHuman() as [null, DecodedReply];
  }
};

export const getDecodedReply = (payload: Bytes, meta?: ProgramMetadata) => {
  const decodedPayload = getDecodedPayload(payload, meta);
  if (decodedPayload) {
    return decodedPayload[1] as DecodedReply;
  }
};
