import { Start, Session, Radar, useLaunchState } from 'features/session';
import { Loader } from 'components';
import styles from './Home.module.scss';

function Home() {
  const state = useLaunchState();
  const { sessionId, currentSession, events } = state || {};

  return (
    <div className={styles.container}>
      {sessionId ? (
        <>
          {currentSession && events && <Session id={sessionId} session={currentSession} events={events} />}
          {currentSession && !events && <Start sessionId={sessionId} session={currentSession} />}
          {!currentSession && <p>Waiting for session to start...</p>}
        </>
      ) : (
        <Loader />
      )}

      <Radar events={events} />
    </div>
  );
}

export { Home };
