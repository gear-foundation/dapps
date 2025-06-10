import { useCallback, useRef } from 'react';

function useThrottle<T extends (...args: Parameters<T>) => ReturnType<T>>(callback: T, delay: number): T {
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

export { useThrottle };
