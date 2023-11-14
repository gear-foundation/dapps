import { Footer } from '@dapps-frontend/ui';
import { BaseComponentProps } from '../../app/types';
import { useIsAppReady, useIsAppReadySync } from '../../app/hooks/use-is-app-ready';
import { useWalletSync } from '../../features/wallet/hooks';
import { useAuthSync, useAutoLogin } from '../../features/auth/hooks';
import { Header } from './header';
import { ApiLoader } from '../loaders';
import { Container } from './container';

type MainLayoutProps = BaseComponentProps;

export function MainLayout({ children }: MainLayoutProps) {
  const { isAppReady } = useIsAppReady();

  useAutoLogin();
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

      <Container>
        <Footer vara />
      </Container>
    </>
  );
}
