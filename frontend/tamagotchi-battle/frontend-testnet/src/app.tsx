import './app.scss'
import { useAccount, useApi } from '@gear-js/react-hooks'
import { Routing } from './pages'
import { ApiLoader, Loader } from '@/components'
import { Footer, Header } from '@/components/layout'
import { withProviders } from '@/app/hocs'
import { LoadingError } from '@/components/loaders/loading-error'
import { useInitBattleData } from '@/features/battle-tamagotchi/hooks'

function Component() {
  const { isApiReady } = useApi()
  const { isAccountReady } = useAccount()
  // const { isFTBalanceReady } = useFTBalance()
  const { isGameReady, errorGame } = useInitBattleData()
  // const { errorFT } = useFTBalanceSync()

  const isAppReady = isApiReady && isAccountReady
  const isUserReady = isGameReady //&& isFTBalanceReady
  const hasError = errorGame //|| errorFT

  return (
    <>
      <Header />
      <main>
        {isAppReady ? (
          <>
            {/*{errorFT && (*/}
            {/*  <LoadingError>*/}
            {/*    <p>Error in the FT contract :(</p>*/}
            {/*    <pre>*/}
            {/*      <small>Error message:</small> <code>{errorFT}</code>*/}
            {/*    </pre>*/}
            {/*  </LoadingError>*/}
            {/*)}*/}
            {errorGame && (
              <LoadingError>
                <p>Error in the Game contract :(</p>
                <pre>
                  <small>Error message:</small> <code>{errorGame}</code>
                </pre>
              </LoadingError>
            )}
            {!hasError && isUserReady && <Routing />}
            {!hasError && !isUserReady && <Loader />}
          </>
        ) : (
          <ApiLoader />
        )}
      </main>
      <Footer />
    </>
  )
}

export const App = withProviders(Component)
