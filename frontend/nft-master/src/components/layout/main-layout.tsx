import { BaseComponentProps } from '../../app/types'
import {
  useIsAppReady,
  useIsAppReadySync,
} from '../../app/hooks/use-is-app-ready'
import { useWalletSync } from '../../features/wallet/hooks'
import { useAuthSync, useAutoLogin } from '../../features/auth/hooks'
import { Header } from './header'
import { Footer } from './footer'
import { ApiLoader } from '../loaders'

type MainLayoutProps = BaseComponentProps

export function MainLayout({ children }: MainLayoutProps) {
  const { isAppReady } = useIsAppReady()

  useAutoLogin()
  useIsAppReadySync()
  useWalletSync()
  useAuthSync()

  return (
    <>
      <Header />
      <main>
        {!isAppReady && <ApiLoader />}
        {isAppReady && children}
      </main>
      <Footer />
    </>
  )
}
