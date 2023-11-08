import 'app.scss'
import { useAccount } from '@gear-js/react-hooks'
import { Loader } from 'components'
import { Routing } from 'pages'
import { withProviders } from 'hocs'
import { useAccountAvailableBalance } from 'features/available-balance/hooks'
import { useNFTSetup } from './features/nfts'
import { usePendingUI } from './hooks'
import { useIsAppReady } from './app/hooks/use-is-app-ready'
import { MainLayout } from './components/layout/main-layout'

function Component() {
  const { isAccountReady } = useAccount()
  const { isAppReady } = useIsAppReady()
  const isNFTReady = useNFTSetup()
  const { isPending } = usePendingUI()
  const { isAvailableBalanceLoading } = useAccountAvailableBalance()

  const isEachStateReady =
    !isPending &&
    !isAvailableBalanceLoading &&
    isNFTReady &&
    isAppReady &&
    isAccountReady

  return (
    <MainLayout>
      {isEachStateReady && <Routing />}
      {!isEachStateReady && <Loader />}
    </MainLayout>
  )
}

export const App = withProviders(Component)
