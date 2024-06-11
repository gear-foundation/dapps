# @dapps-frontend/signless-transactions

## Install

```sh
yarn add @dapps-frontend/signless-transactions
```

## Use

```jsx
import { useSignlessTransactions } from '@dapps-frontend/signless-transactions';

const signlessContext = useSignlessTransactions();

const { pair, session, isSessionReady, voucher, isLoading, setIsLoading, isActive, isSessionActive } = signlessContext;
```
