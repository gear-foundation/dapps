import { useAlert } from '@gear-js/react-hooks';

import { getPanicType } from '@/utils';

import { usePending } from './use-pending';

export type Options = {
  onSuccess?: () => void;
  onError?: (error?: unknown) => void;
};

export function useExecuteWithPending() {
  const { setPending } = usePending();
  const alert = useAlert();

  const executeWithPending = async (action: () => Promise<void>, options?: Options) => {
    try {
      setPending(true);
      await action();
      options?.onSuccess?.();
    } catch (error) {
      console.error(error);
      options?.onError?.(error);

      const panicType = getPanicType(error);
      const alertError = typeof error === 'string' ? error : panicType;

      if (alertError) {
        alert.error(alertError);
      }
    } finally {
      setPending(false);
    }
  };

  return { executeWithPending };
}
