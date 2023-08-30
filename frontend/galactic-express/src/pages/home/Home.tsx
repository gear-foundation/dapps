import { Start, Session, useLaunchState } from 'features/session';
import { Loader } from 'components';

function Home() {
  const state = useLaunchState();
  const { sessionId, currentSession, events } = state || {};
  const isAnyEvent = Object.keys(events || {}).length > 0;

  return sessionId ? (
    <>
      {currentSession && events && isAnyEvent && <Session id={sessionId} session={currentSession} events={events} />}
      {currentSession && !isAnyEvent && <Start sessionId={sessionId} session={currentSession} />}
      {!currentSession && <p>Waiting for session to start...</p>}
    </>
  ) : (
    <Loader />
  );
}

export { Home };
