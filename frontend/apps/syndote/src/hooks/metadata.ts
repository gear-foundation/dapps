import { useEffect, useMemo, useState } from 'react';
import { useAtomValue } from 'jotai';
import { useAccount, useAlert, useReadFullState, useSendMessage, useSendMessageWithGas } from '@gear-js/react-hooks';
import { getStateMetadata, ProgramMetadata, StateMetadata } from '@gear-js/api';
import { HexString } from '@polkadot/util/types';
import meta from 'assets/meta/syndote_meta.txt';
import { ADDRESS } from 'consts';
import { CURRENT_GAME_ADMIN_ATOM } from 'atoms';
import { GameSessionState, State } from 'types';

function useBuffer(source: string) {
  const alert = useAlert();

  const [buffer, setBuffer] = useState<Buffer>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => Buffer.from(arrayBuffer))
      .then((result) => setBuffer(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return buffer;
}

function useProgramMetadata(source: string) {
  const alert = useAlert();

  const [metadata, setMetadata] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.text())
      .then((raw) => `0x${raw}` as HexString)
      .then((metaHex) => ProgramMetadata.from(metaHex))
      .then((result) => setMetadata(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return metadata;
}

function useStateMetadata(wasm: Buffer | undefined) {
  const alert = useAlert();

  const [stateMetadata, setStateMetadata] = useState<StateMetadata>();

  useEffect(() => {
    if (!wasm) return;

    getStateMetadata(wasm)
      .then((result) => setStateMetadata(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [wasm]);

  return stateMetadata;
}

function useSyndoteMessage() {
  const metadata = useProgramMetadata(meta);

  return {
    isMeta: !!meta,
    sendMessage: useSendMessageWithGas(ADDRESS.CONTRACT, metadata, { isMaxGasLimit: true }),
    sendPlayMessage: useSendMessage(ADDRESS.CONTRACT, metadata),
  };
}

function useReadGameSessionState() {
  const { account } = useAccount();
  const metadata = useProgramMetadata(meta);
  const admin = useAtomValue(CURRENT_GAME_ADMIN_ATOM);

  console.log('getting state by', admin || account?.decodedAddress);
  const payload = useMemo(
    () => ({
      GetGameSession: {
        accountId: admin || account?.decodedAddress,
      },
    }),
    [admin, account?.decodedAddress],
  );

  const { state, isStateRead } = useReadFullState<GameSessionState>(ADDRESS.CONTRACT, metadata, payload);

  return { state: state?.GameSession.gameSession, isStateRead };
}

export { useBuffer, useProgramMetadata, useStateMetadata, useSyndoteMessage, useReadGameSessionState };
