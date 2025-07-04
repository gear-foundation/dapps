import { PropsWithChildren } from 'react';

import { Footer } from '@dapps-frontend/ui';

import { useIsAppReady, useIsAppReadySync } from '@/app/hooks/use-is-app-ready';
import { ApiLoader, Header } from '@/components';

import { Container } from '../ui/container';

type MainLayoutProps = PropsWithChildren;

export function MainLayout({ children }: MainLayoutProps) {
  const { isAppReady } = useIsAppReady();

  useIsAppReadySync();

  return (
    <>
      <Header />
      <main>
        {!isAppReady && <ApiLoader />}
        {isAppReady && children}
      </main>

      <Container>
        <Footer vara />
      </Container>
    </>
  );
}
