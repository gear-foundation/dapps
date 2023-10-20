import './global.css'
import './app.scss'
import { useApi, useAccount } from '@gear-js/react-hooks'
import { Routing } from './pages'
import { ApiLoader } from './components/loaders/api-loader'
import { Footer, Header } from '@/components/layout'
import { withProviders } from '@/app/hocs'

const Component = () => {
  const { isApiReady } = useApi()
  const { isAccountReady } = useAccount()
  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <main className="flex flex-col flex-1 container pt-3 pb-5">
        {isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}
      </main>
      <Footer />
    </div>
  )
}

export const App = withProviders(Component)
