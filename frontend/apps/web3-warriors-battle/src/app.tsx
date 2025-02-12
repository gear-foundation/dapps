import './app.scss';
import { withProviders } from '@/app/hocs';
import { Loader, LoadingError, MainLayout } from '@/components';
import '@gear-js/vara-ui/dist/style.css';
import { Routing } from '@/pages';

import { useMyBattleQuery, useProgram } from './app/utils';

function Component() {
  const program = useProgram();

  const { error } = useMyBattleQuery();
  const isGameReady = !!program;

  return (
    <MainLayout>
      {!!error && (
        <LoadingError>
          <p>Error in the Game contract :(</p>
          <pre>
            <small>Error message:</small> <code>{error.message}</code>
          </pre>
        </LoadingError>
      )}
      {!error && isGameReady && <Routing />}
      {!error && !isGameReady && <Loader />}
    </MainLayout>
  );
}

export const App = withProviders(Component);
