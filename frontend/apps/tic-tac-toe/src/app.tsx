import './app.scss';
import { withProviders } from '@/app/hocs';
import { useInitGame, useInitGameSync } from '@/features/tic-tac-toe/hooks';
import meta from '@/features/tic-tac-toe/assets/meta/tic_tac_toe.meta.txt';
import { Routing } from '@/pages';
import { Loader, LoadingError, MainLayout } from '@/components';
import { useProgramMetadata } from './app/hooks';

function Component() {
  const metadata = useProgramMetadata(meta);
  const { isGameReady } = useInitGame();
  const { errorGame: hasError } = useInitGameSync(metadata);

  return (
    <MainLayout>
      {!!hasError && (
        <LoadingError>
          <p>Error in the Game contract :(</p>
          <pre>
            <small>Error message:</small> <code>{hasError}</code>
          </pre>
        </LoadingError>
      )}
      {!hasError && isGameReady && <Routing />}
      {!hasError && !isGameReady && <Loader />}
    </MainLayout>
  );
}

export const App = withProviders(Component);
