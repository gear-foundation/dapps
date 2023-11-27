import { useAccount } from '@gear-js/react-hooks';
import { CURRENT_CONTRACT_ADDRESS_ATOM, IS_CONTRACT_ADDRESS_INITIALIZED_ATOM } from 'atoms';
import { useAtomValue } from 'jotai';
import { Wallet } from '@dapps-frontend/ui';
import { Start, Session, useLaunchState } from 'features/session';
import { Welcome } from 'features/welcome/components/welcome';
import { EnterContractAddress } from 'features/welcome/components/enter-contract-address';

function Home() {
  const { account } = useAccount();
  const currentContractAddress = useAtomValue(CURRENT_CONTRACT_ADDRESS_ATOM);
  const isContractAddressInitialized = useAtomValue(IS_CONTRACT_ADDRESS_INITIALIZED_ATOM);
  const state = useLaunchState(currentContractAddress);
  const { admin, session, turns, participants, isSessionEnded, rankings } = state || {};

  const { sessionId, altitude, weather, reward } = session || {};

  const isUserAdmin = admin === account?.decodedAddress;
  const isStateComing = !!state;

  return (
    <>
      {(!isContractAddressInitialized || (!Number(sessionId) && isSessionEnded)) && (
        <Welcome>
          {account ? (
            <EnterContractAddress
              doesSessionExist={!isSessionEnded}
              isUserAdmin={isUserAdmin}
              isStateComing={isStateComing}
            />
          ) : (
            <Wallet />
          )}
        </Welcome>
      )}
      {currentContractAddress && isContractAddressInitialized && (
        <>
          {participants ? (
            <>
              {!isSessionEnded && (
                <Start
                  participants={participants}
                  session={{
                    altitude: altitude || '',
                    weather: weather || '',
                    reward: reward || '',
                    sessionId: sessionId || '',
                  }}
                  isUserAdmin={isUserAdmin}
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
                      participants={participants}
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
