import { useAlert } from '@gear-js/react-hooks';
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

      const alertError = typeof error === 'string' ? error : error instanceof Error ? error.message : null;

      if (alertError) {
        alert.error(alertError);
      }
    } finally {
      setPending(false);
    }
  };

  return { executeWithPending };
}
