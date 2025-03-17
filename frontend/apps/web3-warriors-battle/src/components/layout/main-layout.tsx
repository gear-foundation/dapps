import { useApi, useAccount } from '@gear-js/react-hooks';
import { PropsWithChildren } from 'react';

import { Footer } from '@dapps-frontend/ui';

import { ApiLoader, Header } from '@/components';

import { Container } from '../ui/container';

type MainLayoutProps = PropsWithChildren;

export function MainLayout({ children }: MainLayoutProps) {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  return (
    <>
      <Header />

      <main>{isApiReady && isAccountReady ? children : <ApiLoader />}</main>

      <Container>
        <Footer vara />
      </Container>
    </>
  );
}
