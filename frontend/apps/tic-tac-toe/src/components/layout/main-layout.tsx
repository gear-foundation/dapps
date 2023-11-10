import { ApiLoader, Footer, Header } from '@/components';
import { PropsWithChildren } from 'react';
import { useIsAppReady, useIsAppReadySync } from '@/app/hooks/use-is-app-ready';
import { useWalletSync } from '@/features/wallet/hooks';
import { useAuthSync } from '@/features/auth/hooks';

type MainLayoutProps = PropsWithChildren;

export function MainLayout({ children }: MainLayoutProps) {
  const { isAppReady } = useIsAppReady();

  useIsAppReadySync();
  useWalletSync();
  useAuthSync();

  return (
    <>
      <Header />
      <main>
        {!isAppReady && <ApiLoader />}
        {isAppReady && children}
      </main>
      <Footer />
    </>
  );
}
