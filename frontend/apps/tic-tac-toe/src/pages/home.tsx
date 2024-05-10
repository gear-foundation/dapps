import { useAccount } from '@gear-js/react-hooks';
import { EzTransactionsSwitch } from '@dapps-frontend/ez-transactions';
import { useGame } from '@/features/tic-tac-toe/hooks';
import { Game, Welcome } from '@/features/tic-tac-toe';
import { WalletNew as Wallet } from '@dapps-frontend/ui';
import { GameStartButton } from '@/features/tic-tac-toe/components/game-start-button';
import metaTxt from '@/features/tic-tac-toe/assets/meta/tic_tac_toe.meta.txt';
import { useProgramMetadata } from '@/app/hooks';
import { Loader } from '@/components';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';

export default function Home() {
  const { account } = useAccount();
  const { gameState } = useGame();
  const meta = useProgramMetadata(metaTxt);

  return meta ? (
    <>
      {gameState ? (
        <Game game={gameState} meta={meta} />
      ) : (
        <Welcome>
          {!account && <Wallet />}
          {!!account && (
            <>
              <GameStartButton meta={meta}>Start the game</GameStartButton>
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
