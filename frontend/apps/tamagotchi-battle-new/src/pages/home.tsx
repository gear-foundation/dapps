import { useAccount } from '@gear-js/react-hooks';
import { WalletNew as Wallet } from '@dapps-frontend/ui';
import { Card, Loader } from '@/components';
import { useProgram } from '@/app/utils';
import { Background, ImportCharacter, SelectGameMode } from '@/features/game/components';
import { useAtomValue } from 'jotai';
import { gameStatusAtom } from '@/features/game/store';
import { GameStatus } from '@/features/game/types';

export default function Home() {
  const { account } = useAccount();
  // const { gameState } = useGame();
  const gameStatus = useAtomValue(gameStatusAtom);
  // const gameStatus = 'import' as GameStatus;
  // const program = useProgram();
  const program = true;

  return program ? (
    <>
      {!account && (
        <Background>
          <Card
            title="Tamagotchi Battle"
            subTitle="Create your Tamagotchi character and engage in battles with other players.">
            <Wallet />
          </Card>
        </Background>
      )}
      {!!account && (
        <>
          {gameStatus === null && <SelectGameMode />}
          {gameStatus === 'import' && <ImportCharacter />}
          {gameStatus === 'generate' && <SelectGameMode />}
          {gameStatus === 'create' && <SelectGameMode />}
          {gameStatus === 'find' && <SelectGameMode />}
        </>
      )}
    </>
  ) : (
    <Loader />
  );
}
