import { useAccount } from '@gear-js/react-hooks';
import { useApp, useGame } from 'app/context';
import { cn } from 'app/utils';
import { LoginSection, GameSection, StartSection, RegistrationSection, FinishedSection, CanceledSection } from 'components/sections';
import { useInitGame } from 'app/hooks/use-game';
import { useEffect, useState } from 'react';

export const Home = () => {
  useInitGame();

  const { account } = useAccount();
  const { game, previousGame, setPreviousGame } = useGame();
  const { setOpenEmptyPopup, openEmptyPopup } = useApp()

  useEffect(() => {
    const isAdmin = previousGame?.admin === account?.decodedAddress;

    if (game) {
      setPreviousGame(game);
    }
    else if (previousGame) {
      if (!isAdmin) {
        setOpenEmptyPopup(true)
      }
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
