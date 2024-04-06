# gasless-transactions

A package for providing gasless transactions

## Install:

```sh
yarn add @dapps-frontend/gasless-transactions
```

## Use

Import GaslessTransactionsProvider from @dapps-frontend/gasless-transactions in your index.tsx and wrap your application. You should pass required arguments in it:

```jsx
import { GaslessTransactionsProvider } from '@dapps-frontend/gasless-transactions';

<GaslessTransactionsProvider
  programId={ADDRESS.GAME} //Program address
  backendAddress={ADDRESS.GASLESS_BACKEND}  //Address of a gasless backend
  voucherLimit={18} //A limit when voucher balance needs to be replenished.
>
  <App>
</GaslessTransactionsProvider>

```

The package provides `useGaslessTransactions` hook which returns a context with all required properties.

### useGaslessTransactions

This hook currently returns two properties:

```jsx
import { useGaslessTransactions } from '@dapps-frontend/gasless-transactions';

const { voucherId, isLoadingVoucher } = useGaslessTransactions();
```

`voucherId` - id of a created voucher for current account

`isLoadingVoucher` - a boolean value indicating whether the voucher is being created/updated at the moment

You can use voucher id to get all required details via methods provided with @gear-js/api
