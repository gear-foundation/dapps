import { HexString } from '@gear-js/api';
import { useApi, useAccount, useAlert } from '@gear-js/react-hooks';
import { web3FromSource } from '@polkadot/extension-dapp';
import { EventRecord } from '@polkadot/types/interfaces';
import { ISubmittableResult } from '@polkadot/types/types';
import type { Event } from '@polkadot/types/interfaces';

function useIssueVoucher() {
  const { api, isApiReady } = useApi();
  const { account } = useAccount();
  const alert = useAlert();

  const getExtrinsicFailedMessage = (event: Event) => {
    if (!isApiReady) throw new Error('API is not initialized');

    const { docs, method: errorMethod } = api.getExtrinsicFailedError(event);
    const formattedDocs = docs.filter(Boolean).join('. ');

    return `${errorMethod}: ${formattedDocs}`;
  };

  const handleEventsStatus = (events: EventRecord[], onSuccess: () => void) => {
    if (!isApiReady) return Promise.reject(new Error('API is not initialized'));

    events.forEach(({ event }) => {
      const { method, section } = event;
      const alertOptions = { title: `${section}.${method}` };

      if (method === 'ExtrinsicFailed') return alert.error(getExtrinsicFailedMessage(event), alertOptions);

      if (method === 'VoucherIssued') {
        alert.success('Voucher issued', alertOptions);
        onSuccess();
      }
    });
  };

  // TODO: sign transaction helper
  const handleEvents = ({ events, status }: ISubmittableResult, onSuccess: () => void) => {
    if (status.isInBlock) return handleEventsStatus(events, onSuccess);
    if (status.isInvalid) alert.error('');
  };

  const issueVoucher = async (programId: HexString, address: HexString, value: string, onSuccess: () => void) => {
    if (!isApiReady || !account) return;

    const { meta } = account;

    try {
      const { extrinsic } = api.voucher.issue(address, programId, value);

      const { signer } = await web3FromSource(meta.source);

      extrinsic.signAndSend(account.address, { signer }, (events) => handleEvents(events, onSuccess));
    } catch (error) {
      if (error instanceof Error) alert.error(error.message);
    }
  };

  return issueVoucher;
}

export { useIssueVoucher };
