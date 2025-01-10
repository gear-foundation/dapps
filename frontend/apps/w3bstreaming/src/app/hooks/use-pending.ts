import { useAtom, atom } from 'jotai';

export const IS_LOADING = atom<boolean>(false);

export function usePending() {
  const [pending, setPending] = useAtom(IS_LOADING);

  return { pending, setPending };
}
