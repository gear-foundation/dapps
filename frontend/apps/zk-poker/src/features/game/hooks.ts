import { useAtom } from 'jotai';

import { pendingAtom } from './store';

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);

  return { pending, setPending };
}
