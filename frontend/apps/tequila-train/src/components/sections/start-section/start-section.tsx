import { Button } from '@gear-js/vara-ui';
import { useState } from 'react';

import { CreateGame } from './create-game';
import { FindGame } from './find-game';
import styles from './start-secrtion.module.scss';

export const StartSection = () => {
  const [isFindGame, setIsFindGame] = useState(false);
  const [isCreateGame, setIsCreateGame] = useState(false);

  if (isCreateGame) {
    return <CreateGame closeCreateGame={() => setIsCreateGame(false)} />;
  }

  return (
    <div className="container my-15 py-32 flex items-center">
      <div className="grow flex space-x-8 justify-between bg-white pr-20 pl-11 py-19 min-h-[330px] rounded-[32px] text-white">
        <div className="relative basis-[220px] lg:basis-[365px] grow-0 shrink-0">
          <div className="absolute -inset-y-10 lg:-top-52 lg:-bottom-21.5 inset-x-0">
            <img
              width={733}
              height={955}
              className="h-full w-full object-contain"
              src="/images/register.webp"
              alt="image"
              loading="lazy"
            />
          </div>
        </div>
        {!isFindGame && (
          <div className="basis-[540px] grow lg:grow-0">
            <h2 className="text-[32px] leading-none font-bold text-black">Welcome to Tequila Train </h2>
            <p className="mt-3 text-[#555756]">
              To begin, choose whether you want to join an existing game or become an administrator and create a new
              game.
            </p>

            <div className="mt-6 flex gap-5">
              <Button
                text="Find game"
                color="primary"
                className={styles.connectButton}
                onClick={() => setIsFindGame(true)}
              />
              <Button
                text="Create game"
                color="contrast"
                className={styles.connectButton}
                onClick={() => setIsCreateGame(true)}
              />
            </div>
          </div>
        )}

        {isFindGame && <FindGame closeFindGame={() => setIsFindGame(false)} />}
      </div>
    </div>
  );
};
