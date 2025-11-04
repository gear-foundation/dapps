import { MessagesDispatched, ProgramMetadata, getStateMetadata } from '@gear-js/api';
import { useAlert, useApi, withoutCommas } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { HexString } from '@polkadot/util/types';
import { useEffect, useState } from 'react';

import { getErrorMessage } from '@dapps-frontend/ui';

import { useLessons, useTamagotchi } from '@/app/context';
import type { TamagotchiState } from '@/app/types/lessons';
import { sleep } from '@/app/utils';
import state2 from '@/assets/meta/state2.meta.wasm?url';

import { useLessonAssets } from '../utils/get-lesson-assets';

import { useStateMetadata } from './use-metadata';

type StateWasmResponse = {
  fed: string;
  entertained: string;
  rested: string;
};

type Args = {
  programId: HexString | undefined;
  wasm: Buffer | Uint8Array | undefined;
  programMetadata: ProgramMetadata | undefined;
  functionName: string | undefined;
  payload?: AnyJson;
  argument?: AnyJson;
};

// TODO: monkey patch, since we need to re-read state every few seconds
// get rid after @gear-js/react-hooks update to return readState callback
function useReadWasmState<T = AnyJson>(args: Args, isReadOnError?: boolean) {
  const { programId, wasm, programMetadata, functionName, payload, argument } = args;

  const { api } = useApi();
  const alert = useAlert();

  const [state, setState] = useState<T>();
  const [isStateRead, setIsStateRead] = useState(true);
  const [errorMessage, setErrorMessage] = useState('');

  const isPayload = payload !== undefined;
  const isArgument = argument !== undefined;

  const readWasmState = (isInitLoad?: boolean) => {
    if (!api || !programId || !wasm || !programMetadata || !functionName || !isArgument || !isPayload) return;

    if (isInitLoad) setIsStateRead(false);

    void getStateMetadata(wasm as Uint8Array)
      .then((stateMetadata) =>
        api.programState.readUsingWasm(
          { programId, wasm: wasm as Uint8Array, fn_name: functionName, argument, payload },
          stateMetadata,
          programMetadata,
        ),
      )
      .then((codecState) => codecState.toHuman())
      .then((result) => {
        setState(result as unknown as T);
        if (!isReadOnError) setIsStateRead(true);
      })
      .catch((error) => setErrorMessage(getErrorMessage(error)))
      .finally(() => {
        if (isReadOnError) setIsStateRead(true);
      });
  };

  const handleStateChange = ({ data }: MessagesDispatched) => {
    const changedIDs = data.stateChanges.toHuman() as HexString[];
    const isAnyChange = changedIDs.some((id) => id === programId);

    if (isAnyChange) readWasmState();
  };

  useEffect(() => {
    if (!api || !programId || !wasm || !programMetadata || !functionName || !isArgument || !isPayload) return;

    const unsub = api.gearEvents.subscribeToGearEvent('MessagesDispatched', handleStateChange);

    return () => {
      void unsub.then((unsubCallback) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, programId, wasm, programMetadata, functionName, argument, payload]);

  useEffect(() => {
    readWasmState(true);
    setErrorMessage('');
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, programId, wasm, programMetadata, functionName, argument, payload]);

  useEffect(() => {
    if (errorMessage) alert.error(errorMessage);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [errorMessage]);

  // contract calculates new state on read without MessagesDispatched events
  useEffect(() => {
    if (!state) return;

    const interval = setInterval(() => readWasmState(), 10000);

    return () => {
      clearInterval(interval);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state]);

  return { state, isStateRead, error: errorMessage, readWasmState };
}

export { useReadWasmState };

export function useThrottleWasmState() {
  const { lesson, setIsReady, isReady } = useLessons();
  const stateMeta = useStateMetadata(state2);
  const [, meta] = useLessonAssets();
  const { tamagotchi, setTamagotchi } = useTamagotchi();

  const { state } = useReadWasmState<StateWasmResponse>({
    programId: lesson?.programId,
    wasm: stateMeta?.buffer,
    programMetadata: meta,
    payload: '0x',
    functionName: 'current_state',
    argument: null,
  });

  useEffect(() => {
    if (!state || !lesson || lesson.step < 2) return;

    const { fed, rested, entertained } = state;

    setTamagotchi({
      ...tamagotchi,
      ...state,
      isDead:
        [+withoutCommas(fed), +withoutCommas(rested), +withoutCommas(entertained)].reduce((sum, a) => sum + +a) === 0,
    } as TamagotchiState);

    void sleep(1).then(() => {
      if (lesson.step > 1 && !isReady) {
        setIsReady(true);
      }
    });

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state, lesson, isReady]);
}
