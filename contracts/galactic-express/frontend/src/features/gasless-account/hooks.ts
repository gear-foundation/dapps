import { GasInfo, ProgramMetadata, decodeAddress } from '@gear-js/api';
import { EventRecord } from '@polkadot/types/interfaces';
import { AnyJson, ISubmittableResult } from '@polkadot/types/types';
import { HexString } from '@polkadot/util/types';
import { useRef } from 'react';
import { DEFAULT_ERROR_OPTIONS, DEFAULT_SUCCESS_OPTIONS, useAlert, useApi } from '@gear-js/react-hooks';
import { bnToBn } from '@polkadot/util';
import { Keyring } from '@polkadot/api';
import { useGaslessAccount } from './Context';

type SendMessageOptions = {
  value?: string | number;
  isOtherPanicsAllowed?: boolean;
  onSuccess?: () => void;
  onError?: () => void;
};

const MAX_GAS_LIMIT = 250000000000;

const getAutoGasLimit = ({ waited, min_limit }: GasInfo) =>
  waited ? min_limit.add(min_limit.mul(bnToBn(0.1))) : min_limit;

function useGaslessSendMessage(destination: HexString, metadata: ProgramMetadata | undefined, isMaxGasLimit = false) {
  const { api } = useApi();
  const { account } = useGaslessAccount();
  const alert = useAlert();

  const title = 'gear.sendMessage';
  const loadingAlertId = useRef('');

  const handleEventsStatus = (events: EventRecord[], onSuccess?: () => void, onError?: () => void) => {
    events.forEach(({ event: { method, section } }) => {
      if (method === 'MessageQueued') {
        alert.success(`${section}.MessageQueued`);
        // eslint-disable-next-line @typescript-eslint/no-unused-expressions
        onSuccess && onSuccess();
      } else if (method === 'ExtrinsicFailed') {
        alert.error('Extrinsic Failed', { title });
        // eslint-disable-next-line @typescript-eslint/no-unused-expressions
        onError && onError();
      }
    });
  };

  const handleStatus = (result: ISubmittableResult, onSuccess?: () => void, onError?: () => void) => {
    const { status, events } = result;
    const { isReady, isInBlock, isInvalid, isFinalized } = status;

    if (isInvalid) {
      alert.update(loadingAlertId.current, 'Transaction error. Status: isInvalid', DEFAULT_ERROR_OPTIONS);
    } else if (isReady) {
      alert.update(loadingAlertId.current, 'Ready');
    } else if (isInBlock) {
      alert.update(loadingAlertId.current, 'In Block');
    } else if (isFinalized) {
      alert.update(loadingAlertId.current, 'Finalized', DEFAULT_SUCCESS_OPTIONS);
      handleEventsStatus(events, onSuccess, onError);
    }
  };

  const sendMessage = (payload: AnyJson, options?: SendMessageOptions) => {
    const { publicKey, privateKey } = account;

    if (publicKey && privateKey && metadata) {
      loadingAlertId.current = alert.loading('Sign In', { title });

      const { value = 0, isOtherPanicsAllowed = false, onSuccess, onError } = options || {};

      const decodedAddress = decodeAddress(publicKey);
      const keyring = new Keyring({ type: 'sr25519' });
      const accountPair = keyring.addFromMnemonic(privateKey);

      const getGasLimit = isMaxGasLimit
        ? Promise.resolve(MAX_GAS_LIMIT)
        : api.program.calculateGas
            .handle(decodedAddress, destination, payload, value, isOtherPanicsAllowed, metadata)
            .then(getAutoGasLimit);

      getGasLimit
        .then((gasLimit) => ({ destination, gasLimit, payload, value }))
        .then((message) => api.message.send(message, metadata))
        .then(() => api.message.signAndSend(accountPair, (result) => handleStatus(result, onSuccess, onError)))
        .catch(({ message }: Error) => {
          alert.update(loadingAlertId.current, message, DEFAULT_ERROR_OPTIONS);
          // eslint-disable-next-line @typescript-eslint/no-unused-expressions
          onError && onError();
        });
    }
  };

  return sendMessage;
}

export { useGaslessSendMessage };
