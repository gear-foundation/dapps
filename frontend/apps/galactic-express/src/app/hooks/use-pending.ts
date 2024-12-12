import { useAtom } from 'jotai';
import { IS_LOADING } from 'atoms';

export function usePending() {
  const [pending, setPending] = useAtom(IS_LOADING);

  return { pending, setPending };
}
