import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { useLounch } from 'app/context';
import { Link } from 'react-router-dom'
import { SessionBoard } from 'components/cards/session-board'
import { LaunchRocketForm } from 'components/forms/launch-rocket-form';
import { LauncheCalc } from 'components/sections/launche-calc'
import { LoginSection } from 'components/sections/login-section';
import { Loader } from 'components/loaders/loader'

export const Home = () => {
  const { sessionIsOver, launch } = useLounch();
  const { account } = useAccount();
  return (
    <section className="grid grid-rows-[1fr_auto_auto] h-[calc(100vh-216px)]" >
      {!launch ? (
        <Loader />
      ) : (
        <>
          {sessionIsOver ? (
            <div className="flex flex-col items-center gap-9 text-center w-full">
              <div className={clsx('w-1/3 panel')}>
                <h2 style={{ color: 'blue' }}>Session is over.</h2>
                <p>Please wait when next launch will start</p>
                <p>
                  <Link to="/launch" style={{ color: 'green' }}>See last launch</Link>
                </p>
              </div>
            </div>
          ) : (
            <>
              <div className="flex flex-col items-center gap-9 text-center w-full">
                <SessionBoard />
              </div>
              <div className={clsx('flex flex-col items-center gap-9', account ? 'mt-4' : 'm-auto')}>
                <div className="flex flex-col items-center gap-9 text-center w-full">
                  <div className="space-y-6 start-launch-container">
                    {account ? (
                      <div>
                        <LauncheCalc />
                        <LaunchRocketForm />
                      </div>
                    ) : (
                      <LoginSection />
                    )}
                  </div>
                </div>
              </div>
            </>
          )}
        </>
      )}


    </section >
  );
};
