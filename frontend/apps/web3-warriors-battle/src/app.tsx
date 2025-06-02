import './app.scss';
import { useApi, useAccount } from '@gear-js/react-hooks';
import { Container, Footer } from '@ui/components';

import { WithProviders } from '@/app/hocs';
import { ApiLoader, Header, Loader, LoadingError } from '@/components';
import { Routing } from '@/pages';

import { useMyBattleQuery, useProgram } from './app/utils';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { error } = useMyBattleQuery();

  const program = useProgram();
  const isGameReady = !!program;

  return (
    <>
      <Header />

      <main>
        {isApiReady && isAccountReady ? (
          <>
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
          </>
        ) : (
          <ApiLoader />
        )}
      </main>

      <Container>
        <Footer vara />
      </Container>
    </>
  );
}

export const App = WithProviders(Component);
