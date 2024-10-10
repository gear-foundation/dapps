import { atom, useAtom } from 'jotai';

const pendingAtom = atom<boolean>(false);

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);

  return { pending, setPending };
}
