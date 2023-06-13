import { useReadFullState } from '@gear-js/react-hooks';
import metaTxt from 'assets/state/launch_site.meta.txt';
import { ADDRESS } from 'consts';
import { useProgramMetadata } from 'hooks';
import { useMemo } from 'react';
import { SessionState } from './types';

function useSessionState() {
  const meta = useProgramMetadata(metaTxt);
  const { state } = useReadFullState<SessionState>(ADDRESS.CONTRACT, meta);

  return state;
}

function useProbability(_weather: string, _payload: string, _fuel: string) {
  const probability = useMemo(() => {
    const weather = +_weather;
    const payload = +_payload;
    const fuel = +_fuel;

    let result = ((((97 / 100) * (95 - weather)) / 100) * (90 - weather)) / 100;

    if (payload >= 80) result = ((((97 / 100) * (85 - 2 * weather)) / 100) * (90 - weather)) / 100;

    if (fuel >= 80) result = (((((87 - 2 * weather) / 100) * (95 - weather)) / 100) * (90 - weather)) / 100;

    if (fuel >= 80 && payload >= 80)
      result = (((((87 - 2 * weather) / 100) * (85 - 2 * weather)) / 100) * (90 - weather)) / 100;

    return Math.round(result * 100);
  }, [_weather, _payload, _fuel]);

  const probabilityId = useMemo(() => {
    if (probability <= 35) return 'low';
    if (probability <= 70) return 'medium';

    return 'high';
  }, [probability]);

  return { probability, probabilityId };
}

export { useSessionState, useProbability };
