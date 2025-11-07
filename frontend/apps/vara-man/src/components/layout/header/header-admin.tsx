import { useEzTransactions } from 'gear-ez-transactions';

import { useApp } from '@/app/context/ctx-app';
import { useGame } from '@/app/context/ctx-game';
import { useChangeStatusMessage } from '@/app/utils';
import { Icons } from '@/components/ui/icons';

export function HeaderAdmin() {
  const { isPending, setIsPending } = useApp();
  const { status } = useGame();

  const { gasless } = useEzTransactions();
  const { changeStatusMessage } = useChangeStatusMessage();

  const onError = () => setIsPending(false);
  const onSuccess = () => setIsPending(false);

  const onActivateGame = () => {
    if (!gasless.isLoading) {
      setIsPending(true);
      void changeStatusMessage({ startedWithNativeToken: null }, { onError, onSuccess });
    }
  };

  const onDeactivateGame = () => {
    if (!gasless.isLoading) {
      setIsPending(true);
      void changeStatusMessage({ paused: null }, { onError, onSuccess });
    }
  };

  return (
    <>
      {status === 'Paused' && (
        <button type="button" className="btn btn--primary px-6" disabled={isPending} onClick={onActivateGame}>
          <Icons.gameJoystick />
          <span>Activate game</span>
        </button>
      )}
      {status === 'Started' && (
        <button type="button" className="btn btn--theme-red px-6" disabled={isPending} onClick={onDeactivateGame}>
          <Icons.gameJoystick />
          <span>Deactivate game</span>
        </button>
      )}
    </>
  );
}
