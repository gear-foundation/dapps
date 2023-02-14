import { PlayerTrackSection } from '../player-track-section';
import { PlayerCardSection } from '../player-card-section';
import { PlayerConsSection } from '../player-cons-section';
import { useApp, useGame } from 'app/context';
import { useEffect } from 'react';

export const GameSection = () => {
  const { gameWasm: state, players } = useGame();
  const { isAllowed } = useApp();

  useEffect(() => {
    console.log({ isAllowed });
  }, [isAllowed]);

  return (
    <div className="container-xl flex flex-col grow">
      <ul className="space-y-px">
        <li>
          <PlayerTrackSection index={-1} train tiles={state ? [state?.startTile] : undefined} />
        </li>
        {state?.tracks.map((p, i) => (
          <li key={i}>
            <PlayerTrackSection index={i} isUserTrain={p.hasTrain} />
          </li>
        ))}
      </ul>
      <div className="grid gap-4 mt-auto">
        {isAllowed && <PlayerConsSection />}
        <ul className="flex gap-4 justify-center">
          {players.map((p, i) => (
            <li key={i}>
              <PlayerCardSection index={i} />
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};
