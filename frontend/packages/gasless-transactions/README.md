# gasless-transactions

A package for providing gasless transactions

## Install:

```sh
yarn add @dapps-frontend/gasless-transactions
```

## Use

Import initGaslessTransactions function from @dapps-frontend/gasless-transactions in your utils and execute it to get requred tools for using gasless transactions. You should pass required arguments in it:

```jsx
import { initGasslessTransactions } from '@dapps-frontend/gasless-transactions';

export const gaslessTransactions = initGasslessTransactions({
  programId: // Contract address
  backendAddress: // Address of the backend managing gasless transactions handling
  voucherLimit?: // OPTIONAL. A limit when voucher balance needs to be replenished. voucherLimit is 18 by default
});

```

An object returned from `initGasslessTransactions` contains a set of tools for handling gasless transactrions.

### useFetchVoucher

This hook creates a voucher for current account and automaticly updates its balance when it becomes lower than `voucherLimit`.

```jsx
const { useFetchVoucher } = gaslessTransactions;

const { isVoucher, isLoading, updateBalance } = useFetchVoucher();
```

`isVoucher` is the boolean variable which shows if a voucher does exist for this account or not

`isLoading` is true if a voucher is creating or updating the balance in this moment

`updateBalance` allows to manually update voucher balance if needed
