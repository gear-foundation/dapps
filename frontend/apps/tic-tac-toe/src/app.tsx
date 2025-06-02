import './app.scss';
import { withProviders } from '@/app/hocs';
import { Loader, LoadingError, MainLayout } from '@/components';
import { useInitGame, useInitGameSync } from '@/features/tic-tac-toe/hooks';
import { Routing } from '@/pages';

function Component() {
  const { isGameReady } = useInitGame();
  const { errorGame } = useInitGameSync();

  return (
    <MainLayout>
      {!!errorGame && (
        <LoadingError>
          <p>Error in the Game contract :(</p>
          <pre>
            <small>Error message:</small> <code>{errorGame.message}</code>
          </pre>
        </LoadingError>
      )}
      {!errorGame && isGameReady && <Routing />}
      {!errorGame && !isGameReady && <Loader />}
    </MainLayout>
  );
}

export const App = withProviders(Component);
