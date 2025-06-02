import { useAlert } from '@gear-js/react-hooks';

import { getErrorMessage } from '@dapps-frontend/ui';

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
      console.log('success');
      options?.onSuccess?.();
    } catch (error) {
      console.error(error);
      options?.onError?.(error);
      alert.error(getErrorMessage(error));
    } finally {
      setPending(false);
    }
  };

  return { executeWithPending };
}
