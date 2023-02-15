import { useAccount } from '@gear-js/react-hooks';
import { LoginSection } from 'components/sections/login-section';
import { GameSection } from '../components/sections/game-section';
import clsx from 'clsx';
import { useInitGame, useWasmState } from '../app/hooks/use-game';
import * as React from 'react';
export const Home = () => {
  useInitGame();
  useWasmState();

  const { account } = useAccount();
  return (
    <section className={clsx('grid grow', !account && 'place-items-center')}>
      {account ? (
        <GameSection />
      ) : (
        <div className="flex flex-col items-center justify-center gap-9 grow">
          <p>Connect your account to start the game</p>
          <LoginSection />
        </div>
      )}
    </section>
  );
};
