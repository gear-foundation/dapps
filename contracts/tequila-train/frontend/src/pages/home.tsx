import { useAccount } from '@gear-js/react-hooks';
import { LoginSection } from 'components/sections/login-section';

export const Home = () => {
  const { account } = useAccount();
  return (
    <section className="grid place-items-center gap-9 grow">
      <div className="space-y-6 ">
        {!account && (
          <div className="">
            <p>Connect your account to start the game</p>
          </div>
        )}
        <div className="flex justify-center">{account ? 'game' : <LoginSection />}</div>
      </div>
    </section>
  );
};
