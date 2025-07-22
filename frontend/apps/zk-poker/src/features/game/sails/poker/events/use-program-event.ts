// ! TODO: remove this file after updating @gear-js/react-hooks
import { UseProgramEventParameters } from '@gear-js/react-hooks';
import {
  EventReturn,
  FunctionName,
  ServiceName,
  Event,
  EventCallbackArgs,
} from '@gear-js/react-hooks/dist/hooks/sails/types';
import { useEffect } from 'react';

function useProgramEvent<
  TProgram,
  TServiceName extends ServiceName<TProgram>,
  TFunctionName extends FunctionName<TProgram[TServiceName], EventReturn>,
  TEvent extends Event<TProgram[TServiceName][TFunctionName]>,
  TCallbackArgs extends EventCallbackArgs<TEvent>,
>({
  program,
  serviceName,
  functionName,
  onData,
  // ! TODO: add queryKey to the @gear-js/react-hooks
  queryKey,
}: UseProgramEventParameters<TProgram, TServiceName, TFunctionName, TCallbackArgs> & {
  queryKey?: unknown[];
}) {
  // depends on useProgram/program implementation, programId may not be available
  const programId = program && typeof program === 'object' && 'programId' in program ? program.programId : undefined;

  useEffect(() => {
    if (!program) return;

    const unsub = (program[serviceName][functionName] as TEvent)(onData) as EventReturn;
    return () => {
      // eslint-disable-next-line @typescript-eslint/no-floating-promises -- TODO(#1816): resolve eslint comments
      unsub.then((unsubCallback) => {
        unsubCallback();
      });
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [programId, serviceName, functionName, ...(queryKey || [])]);
}

export { useProgramEvent };
export type { UseProgramEventParameters };
