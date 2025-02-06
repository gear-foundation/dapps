import { Footer } from '@dapps-frontend/ui';

import { useIsAppReady, useIsAppReadySync } from '../../app/hooks/use-is-app-ready';
import { BaseComponentProps } from '../../app/types';
import { useAuthSync, useAutoLogin } from '../../features/auth/hooks';
import { ApiLoader } from '../loaders';

import { Container } from './container';
import { Header } from './header';

type MainLayoutProps = BaseComponentProps;

export function MainLayout({ children }: MainLayoutProps) {
  const { isAppReady } = useIsAppReady();

  useAutoLogin();
  useIsAppReadySync();
  useAuthSync();

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
