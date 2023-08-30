import { useEffect, useState } from 'react';

/**
 * Hook that tracks state of a CSS media query.
 */
export function useMedia(query: string, defaultState: boolean = false) {
  const [state, setState] = useState(typeof window !== 'undefined' ? window.matchMedia(query).matches : defaultState);
  const listener = (event: MediaQueryListEvent): void => setState(event.matches);

  useEffect(() => {
    const mqList = window.matchMedia(query);
    mqList.addEventListener('change', listener);

    return (): void => mqList.removeEventListener('change', listener);
  }, [query]);

  return state;
}

export const useIsSmall = () => useMedia('(min-width: 480px)');
export const useIsTablet = () => useMedia('(min-width: 768px)');
export const useIsLarge = () => useMedia('(min-width: 1540px)');
