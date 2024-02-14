import { useAccount } from '@gear-js/react-hooks';
import { CURRENT_GAME_ATOM, IS_CONTRACT_ADDRESS_INITIALIZED_ATOM } from 'atoms';
import { useAtomValue } from 'jotai';
import { Start, Session, useLaunchState } from 'features/session';
import { Welcome } from 'features/welcome/components/welcome';
import { RequestGame } from 'features/welcome/components/enter-contract-address';

function Home() {
  const { account } = useAccount();
  const currentGame = useAtomValue(CURRENT_GAME_ATOM);
  const isContractAddressInitialized = useAtomValue(IS_CONTRACT_ADDRESS_INITIALIZED_ATOM);
  const state = useLaunchState();
  const { admin, stage, sessionId, altitude, weather, reward, bid } = state || {};

  const isSessionEnded = Object.keys(stage || {})[0] === 'Results';

  const rankings = stage?.Results?.rankings;
  const turns = stage?.Results?.turns;
  const participants = stage?.Registration;

  const isUserAdmin = admin === account?.decodedAddress;
  const isStateComing = !!state;
  console.log(state);
  return (
    <>
      {(!state || (!Number(sessionId) && isSessionEnded)) && (
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
                    />
                  ) : (
                    <div>The session has passed. You are not participating in this one</div>
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
