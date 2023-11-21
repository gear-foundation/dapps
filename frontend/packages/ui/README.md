# @dapps-frontend/ui

## Install

```sh
yarn add @dapps-frontend/ui
```

## Use

### Container

Main layout container with max width and padding:

```jsx
import { Container } from '@dapps-frontend/ui';

function Header() {
  return (
    <header>
      <Container>Logo and navigation</Container>
    </header>
  );
}

export { Header };
```

### Footer

Main layout footer:

```jsx
import { Footer } from '@dapps-frontend/ui';

function Layout() {
  return (
    <>
      <header />
      <main />
      <Footer />
    </>
  );
}

export { Layout };
```
