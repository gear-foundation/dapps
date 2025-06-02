import { useAccount } from '@gear-js/react-hooks';
import { EzTransactionsSwitch } from 'gear-ez-transactions';

import { Wallet } from '@dapps-frontend/ui';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { useProgram } from '@/app/utils';
import { Loader } from '@/components';
import { Game, Welcome } from '@/features/tic-tac-toe';
import { GameStartButton } from '@/features/tic-tac-toe/components/game-start-button';
import { useGame } from '@/features/tic-tac-toe/hooks';

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
