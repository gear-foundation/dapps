import { useAccount } from '@gear-js/react-hooks';
import { useApp, useGame } from 'app/context';
import { cn } from 'app/utils';
import { LoginSection, GameSection, StartSection, RegistrationSection, FinishedSection, CanceledSection } from 'components/sections';
import { useInitGame } from 'app/hooks/use-game';
import { useEffect } from 'react';

export const Home = () => {
  useInitGame();

  const { account } = useAccount();
  const { game, previousGame, setPreviousGame } = useGame();
  const { setOpenEmptyPopup, openEmptyPopup, isUserCancelled, setIsUserCancelled } = useApp()

  useEffect(() => {
    const isAdmin = previousGame?.admin === account?.decodedAddress;

    if (game) {
      console.log('useEffect game: ', game)
      setPreviousGame(game);
    }
    else if (previousGame) {
      if (!isAdmin && !isUserCancelled) {
        console.log('isUserCancelled', isUserCancelled)
        setOpenEmptyPopup(true)
      }
      setIsUserCancelled(false)
      setPreviousGame(null)
    }
  }, [game]);

  const renderSection = () => {
    if (!account) {
      return <LoginSection />
    }

    if (game?.isStarted || previousGame?.isStarted) {
      return <GameSection />;
    } else if (game?.state && 'Registration') {
      return <RegistrationSection />;
    } else {
      return <StartSection />;
    }
  };

  return (
    <section className={cn('grid grow', !account && 'place-items-center')}>
      {renderSection()}
      {openEmptyPopup && <CanceledSection />}
    </section>
  );
};
