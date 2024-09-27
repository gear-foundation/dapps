import './app.scss';
import { withProviders } from '@/app/hocs';
import { Loader, LoadingError, MainLayout } from '@/components';
import '@gear-js/vara-ui/dist/style.css';
import { Routing } from '@/pages';

function Component() {
  // const { isGameReady } = useInitGame();
  // const { errorGame } = useInitGameSync();

  const program = true;

  const errorGame = false;
  const isGameReady = program;

  return (
    <MainLayout>
      {!!errorGame && (
        <LoadingError>
          <p>Error in the Game contract :(</p>
          <pre>
            <small>Error message:</small> <code>{errorGame}</code>
            {/* <small>Error message:</small> <code>{errorGame.message}</code> */}
          </pre>
        </LoadingError>
      )}
      {!errorGame && isGameReady && <Routing />}
      {!errorGame && !isGameReady && <Loader />}
    </MainLayout>
  );
}

export const App = withProviders(Component);
