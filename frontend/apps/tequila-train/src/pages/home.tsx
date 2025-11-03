import { useAccount } from '@gear-js/react-hooks';
import { useEffect } from 'react';

import { useApp, useGame } from '@/app/context';
import { useInitGame } from '@/app/hooks/use-game';
import { cn } from '@/app/utils';
import { LoginSection, GameSection, StartSection, RegistrationSection, CanceledSection } from '@/components/sections';

export const Home = () => {
  useInitGame();

  const { account } = useAccount();
  const { game, previousGame, setPreviousGame } = useGame();
  const { setOpenEmptyPopup, openEmptyPopup, isUserCancelled, setIsUserCancelled } = useApp();

  useEffect(() => {
    const isAdmin = previousGame?.admin === account?.decodedAddress;

    if (game) {
      setPreviousGame(game);
    } else if (previousGame) {
      if (!isAdmin && !isUserCancelled && !previousGame.state.Winners) {
        setOpenEmptyPopup(true);
      }
      setIsUserCancelled(false);
      setPreviousGame(null);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [game, account?.decodedAddress]);

  const renderSection = () => {
    if (!account) {
      return <LoginSection />;
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
