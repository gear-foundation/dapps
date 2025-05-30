import { useAtom } from 'jotai';
import { atomWithStorage } from 'jotai/utils';
import { useCallback, useRef } from 'react';

import { pendingAtom } from './store';

const USER_NAME_KEY = 'user_name';
const userNameAtom = atomWithStorage(USER_NAME_KEY, 'Player');

export function useUserName() {
  const [userName, setUserName] = useAtom(userNameAtom);
  return { userName, setUserName };
}

// ! TODO: @deprecated
export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);

  return { pending, setPending };
}

export function useThrottle<T extends (...args: Parameters<T>) => ReturnType<T>>(callback: T, delay: number): T {
  const lastRun = useRef(Date.now());
  const timeoutRef = useRef<NodeJS.Timeout>();

  return useCallback(
    (...args: Parameters<T>) => {
      const now = Date.now();

      if (now - lastRun.current >= delay) {
        callback(...args);
        lastRun.current = now;
      } else {
        if (timeoutRef.current) {
          clearTimeout(timeoutRef.current);
        }

        timeoutRef.current = setTimeout(() => {
          callback(...args);
          lastRun.current = Date.now();
        }, delay);
      }
    },
    [callback, delay],
  ) as T;
}
