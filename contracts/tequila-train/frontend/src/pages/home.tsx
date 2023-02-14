import { useAccount } from '@gear-js/react-hooks';
import { LoginSection } from 'components/sections/login-section';
import { GameSection } from '../components/sections/game-section';
import clsx from 'clsx';

export const Home = () => {
  const { account } = useAccount();
  return (
    <section className={clsx('grid gap-9 grow', !account && 'place-items-center')}>
      {!account && (
        <div className="">
          <p>Connect your account to start the game</p>
        </div>
      )}
      {account ? <GameSection /> : <LoginSection />}
    </section>
  );
};
