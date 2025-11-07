import { useCallback, useEffect, useState } from 'react';

type UseMediaQueryOptions = {
  defaultValue?: boolean;
  initializeWithValue?: boolean;
};

const IS_SERVER = typeof window === 'undefined';

export function useMediaQuery(
  query: string,
  { defaultValue = false, initializeWithValue = true }: UseMediaQueryOptions = {},
): boolean {
  const getMatches = useCallback(
    (mediaQuery: string): boolean => {
      if (IS_SERVER) {
        return defaultValue;
      }

      return window.matchMedia(mediaQuery).matches;
    },
    [defaultValue],
  );

  const [matches, setMatches] = useState<boolean>(() => {
    if (initializeWithValue) {
      return getMatches(query);
    }
    return defaultValue;
  });

  // Handles the change event of the media query.
  const handleChange = useCallback(() => {
    setMatches(getMatches(query));
  }, [getMatches, query]);

  useEffect(() => {
    const matchMedia = window.matchMedia(query);

    // Triggered at the first client-side load and if query changes
    handleChange();

    // Use deprecated `addListener` and `removeListener` to support Safari < 14 (#135)
    if (matchMedia.addListener) {
      matchMedia.addListener(handleChange);
    } else {
      matchMedia.addEventListener('change', handleChange);
    }

    return () => {
      if (matchMedia.removeListener) {
        matchMedia.removeListener(handleChange);
      } else {
        matchMedia.removeEventListener('change', handleChange);
      }
    };
  }, [handleChange, query]);

  return matches;
}
