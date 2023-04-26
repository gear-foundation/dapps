import { useAccount } from '@gear-js/react-hooks';
import { LoginSection } from 'components/sections/login-section';
import { GameSection } from '../components/sections/game-section';
import { useInitGame, useWasmState } from '../app/hooks/use-game';
import { useGame } from '../app/context';
import { RegistrationSection } from '../components/sections/registration-section';
import { Loader } from '../components/loaders/loader';
import { cn } from '../app/utils';

export const Home = () => {
  useInitGame();
  useWasmState();

  const { account } = useAccount();
  const { game, gameWasm } = useGame();

  return (
    <section className={cn('grid grow', !account && 'place-items-center')}>
      {account ? (
        game && <>{game.isStarted ? gameWasm ? <GameSection /> : <Loader /> : <RegistrationSection />}</>
      ) : (
        <div className="flex flex-col items-center justify-center gap-9 grow">
          <p>Connect your account to start the game</p>
          <LoginSection />
        </div>
      )}
    </section>
  );
};
