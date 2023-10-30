import { ApiLoader, Footer, Header } from '@/components'
import { PropsWithChildren } from 'react'
import { useIsAppReady, useIsAppReadySync } from '@/app/hooks/use-is-app-ready'
import { useWalletSync } from '@/features/wallet/hooks'
import { useAuthSync } from '@/features/auth/hooks'
import { useNodesSync } from '@/app/hooks/use-nodes-sync'

type MainLayoutProps = PropsWithChildren

export function MainLayout({ children }: MainLayoutProps) {
  const { isAppReady } = useIsAppReady()

  useIsAppReadySync()
  useWalletSync()
  useAuthSync()
  useNodesSync()

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
