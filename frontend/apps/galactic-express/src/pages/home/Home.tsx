import { useAccount } from '@gear-js/react-hooks';
import { Start, Session, useLaunchState } from 'features/session';
import { Welcome } from 'features/welcome/components/welcome';
import { RequestGame } from 'features/welcome/components/enter-contract-address';
import { SessionPassedInfo } from 'features/session/components/session-passed-info';

function Home() {
  const { account } = useAccount();
  const state = useLaunchState();
  const { admin, stage, sessionId, altitude, weather, reward, bid, adminName } = state || {};

  const isSessionEnded = Object.keys(stage || {})[0] === 'Results';

  const rankings = stage?.Results?.rankings;
  const turns = stage?.Results?.turns;
  const participants = stage?.Registration || stage?.Results?.participants;

  const isUserAdmin = admin === account?.decodedAddress;

  return (
    <>
      {!state && !isSessionEnded && (
        <Welcome>
          <RequestGame />
        </Welcome>
      )}
      {!!state && (
        <>
          {true ? (
            <>
              {!isSessionEnded && (
                <Start
                  participants={participants || []}
                  session={{
                    altitude: altitude || '',
                    weather: weather || '',
                    reward: reward || '',
                    sessionId: sessionId || '',
                  }}
                  bid={bid}
                  isUserAdmin={isUserAdmin}
                  adminAddress={admin}
                  adminName={adminName || ''}
                  userAddress={account?.address || ''}
                />
              )}
              {isSessionEnded && (
                <>
                  {rankings?.map((item) => item[0]).includes(account?.decodedAddress || '0x') ? (
                    <Session
                      session={{
                        altitude: altitude || '',
                        weather: weather || '',
                        reward: reward || '',
                        sessionId: sessionId || '',
                      }}
                      participants={participants || []}
                      turns={turns || []}
                      rankings={rankings || []}
                      userId={account?.decodedAddress}
                      admin={admin}
                    />
                  ) : (
                    <SessionPassedInfo />
                  )}
                </>
              )}
            </>
          ) : (
            <p>Waiting for session to start...</p>
          )}
        </>
      )}
    </>
  );
}

export { Home };
