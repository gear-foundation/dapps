import { Modal } from '@/components/ui/modal/modal2';
import { COINS } from '@/feature/game/consts';
import { Button } from '@gear-js/vara-ui';
import { useAtom } from 'jotai';

type GamePlayAgainModalProps = {
  setIsOpenPlayAgain: (_: boolean) => void;
  restartGame: () => void;
};

export const GamePlayAgainModal = ({ setIsOpenPlayAgain, restartGame }: GamePlayAgainModalProps) => {
  const [, setCoins] = useAtom(COINS);

  return (
    <Modal open={true}>
      <Modal.Content>
        <div className="flex flex-col items-center">
          <h2 className="typo-h2">Game over</h2>
          <div className="flex flex-col gap-5 mt-5">
            <p className="text-[#555756]">You're doing great, keep it up!</p>
            <div className="flex gap-10">
              <Button
                text="Play Again"
                className="w-full"
                onClick={() => {
                  setIsOpenPlayAgain(false);
                  restartGame();
                  setCoins({ gold: 0, silver: 0 });
                }}
              />
            </div>
          </div>
        </div>
      </Modal.Content>
    </Modal>
  );
};
