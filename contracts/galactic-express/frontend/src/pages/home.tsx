import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { SessionBoard } from 'components/cards/session-board'
import { LaunchRocketForm } from 'components/forms/launch-rocket-form';
import { LauncheCalc } from 'components/sections/launche-calc'
import { LoginSection } from 'components/sections/login-section';

export const Home = () => {
  const { account } = useAccount();
  return (
    <section className="grid grid-rows-[1fr_auto_auto] h-[calc(100vh-216px)]" >
      <div className="flex flex-col items-center gap-9 text-center w-full">
        <SessionBoard />
      </div>
      <div className={clsx('flex flex-col items-center gap-9', account ? 'mt-12' : 'm-auto')}>
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
    </section >
  );
};
