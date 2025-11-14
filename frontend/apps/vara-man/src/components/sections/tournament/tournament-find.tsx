import { decodeAddress, HexString } from '@gear-js/api';
import { Input, Button } from '@gear-js/vara-ui';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { useGame } from '@/app/context/ctx-game';
import { GameFindModal } from '@/feature/tournament-game/components/modals/game-find';
import { GameNotFoundModal } from '@/feature/tournament-game/components/modals/game-not-found';

type FindGame = {
  admin: HexString;
  bid: bigint;
  participants: number;
};

export const TournamentFind = () => {
  const [findAddress, setFindAddress] = useState('');
  const [foundGame, setFoundGame] = useState<FindGame>();
  const [isOpenFindModal, setIsOpenFindModal] = useState(false);
  const [isOpenNotFound, setIsOpenNotFound] = useState(false);

  const navigate = useNavigate();
  const { allGames } = useGame();

  const onSearchGame = () => {
    if (findAddress) {
      const game = allGames?.find((storedGame) => {
        return storedGame[0] === decodeAddress(findAddress);
      });
      if (game) {
        setIsOpenFindModal(true);
        setFoundGame({
          admin: decodeAddress(findAddress),
          bid: BigInt(game?.[1].bid || 0),
          participants: game[1].participants.length,
        });
      } else {
        setIsOpenNotFound(true);
      }
    }
  };

  return (
    <div className="flex flex-col gap-5 md:justify-center items-center  grow h-full">
      {isOpenFindModal && foundGame && <GameFindModal findGame={foundGame} setIsOpenFindModal={setIsOpenFindModal} />}
      {isOpenNotFound && <GameNotFoundModal setIsOpenFindModal={setIsOpenNotFound} />}

      <h2 className="text-[34px]/[37px] font-semibold text-center md:text-left">Find a private game</h2>
      <p className="text-center md:text-left">To find the game, you need to enter the administrator&apos;s address.</p>

      <form className="grid gap-4 w-full max-w-[600px] mx-auto mt-5">
        <div className="flex flex-col gap-10">
          <Input
            type="text"
            placeholder="kG…"
            label="Specify the game admin address:"
            required
            className="w-full"
            onChange={(e) => setFindAddress(e.target.value)}
          />

          <div className="flex gap-5 flex-col md:flex-row">
            <Button color="grey" text="Back" className="w-full order-1 md:order-none" onClick={() => navigate(-1)} />
            <Button text="Continue" className="w-full" onClick={onSearchGame} />
          </div>
        </div>
      </form>
    </div>
  );
};
