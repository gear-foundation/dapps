import { IS_LOADING } from 'atoms';
import { useAtom } from 'jotai';

export function usePending() {
  const [pending, setPending] = useAtom(IS_LOADING);

  return { pending, setPending };
}
