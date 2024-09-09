import { useAccount } from '@gear-js/react-hooks';
import { EzTransactionsSwitch } from '@dapps-frontend/ez-transactions';
import { useGame } from '@/features/tic-tac-toe/hooks';
import { Game, Welcome } from '@/features/tic-tac-toe';
import { WalletNew as Wallet } from '@dapps-frontend/ui';
import { GameStartButton } from '@/features/tic-tac-toe/components/game-start-button';
import { Loader } from '@/components';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { useProgram } from '@/app/utils';

export default function Home() {
  const { account } = useAccount();
  const { gameState } = useGame();
  const program = useProgram();

  return program ? (
    <>
      {gameState ? (
        <Game game={gameState} />
      ) : (
        <Welcome>
          {!account && <Wallet />}
          {!!account && (
            <>
              <GameStartButton>Start the game</GameStartButton>
              <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
            </>
          )}
        </Welcome>
      )}
    </>
  ) : (
    <Loader />
  );
}
