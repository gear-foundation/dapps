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

### StartDisclaimer

Block with get started description and links to get wasm/metadata files with wiki instructions.

`fileName` is a file name without extension for wasm/meta in a [nightly github releases](https://github.com/gear-foundation/dapps/releases/tag/nightly).

`wikiPath` is a path to `how-to-run` section of app's [Wiki example page](https://wiki.gear-tech.io/docs/examples/prerequisites).

Provided example is valid for Tamagotchi contract with file names `tamagotchi.opt.wasm`/`tamagotchi.meta.txt` and Wiki page `https://wiki.gear-tech.io/docs/examples/Gaming/tamagotchi#how-to-run`

```jsx
import { StartDisclaimer } from '@dapps-frontend/ui';

function Home() {
  return (
    <>
      <button type="button">Start Game</button>
      <StartDisclaimer fileName="tamagotchi" wikiPath="Gaming/tamagotchi" />
    </>
  );
}

export { Home };
```
