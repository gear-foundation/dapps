# gear-ez-transactions

A library to provide gasless and signless transactions.
By interacting with a Gear program via voucher, gasless backend and local account it allows users to make transactions without paying gas fees or signing on-chain transactions.

## Install:

```sh
yarn add gear-ez-transactions
```

## Gasless-transactions

The gas fees, which are usually required to execute transactions on the blockchain, are covered by a [gasless backend service](https://github.com/gear-foundation/dapps/tree/master/backend/gasless) provided by the dApp developer. When a user initiates a transaction, the backend issue a [voucher](https://wiki.vara.network/docs/api/vouchers) for that specific user. This voucher effectively covers the gas cost for the transaction, allowing the user to execute it without having to pay any fees themselves.

### Provider

Import `GaslessTransactionsProvider` from `gear-ez-transactions` in your `index.tsx` and wrap your application with it. You should pass the required arguments:

```jsx
import { GaslessTransactionsProvider } from 'gear-ez-transactions';

<GaslessTransactionsProvider
  programId={'0x...'} // Program address
  backendAddress={'https://.../'}  // URL-address of the gasless backend
  voucherLimit={18} // Limit at which the voucher balance needs to be topped up
>
  <App>
</GaslessTransactionsProvider>
```

### useGaslessTransactions

The package provides a `useGaslessTransactions` hook that returns a context with all required properties:

```jsx
import { useGaslessTransactions } from 'gear-ez-transactions';

const gaslessContext = useGaslessTransactions();
const { voucherId, isLoading, isEnabled, isActive, expireTimestamp, requestVoucher, setIsEnabled } = gaslessContext;
```

`voucherId` - id of a created voucher for current account.

`isLoading` - a boolean value indicating whether the voucher is being created/updated at the moment.

`isEnabled` - a boolean indicating whether the gasless transaction feature is currently enabled (either by user or programmatically).

`isActive` - a boolean indicating whether the gasless transaction is currently active. This typically means that a voucher has been successfully created and is ready for use.

`expireTimestamp` - a timestamp indicating when the voucher will expire.

`requestVoucher` - a function to request the creation of a new voucher. This function typically triggers the process of creating a voucher and is used to initiate gasless transactions.

`setIsEnabled` - a function to toggle the isEnabled state. This can be used to programmatically enable or disable the gasless transaction feature within your application.

You can use `voucherId` to get all required details via methods provided with `@gear-js/api`.

## Use signless-transactions

To streamline the process further, the frontend of the application creates a temporary sub-account for the user. This sub-account is granted the necessary permissions by the user to automatically sign transactions on their behalf. This means that users don’t need to manually sign each transaction with their private key, enhancing convenience.
The main account issue a [voucher](https://wiki.vara.network/docs/api/vouchers) to the sub-account to cover gas fees.

Signless transactions require the implementation of a session service for a program.

The provider can utilize either a [Sails-generated program](https://github.com/gear-tech/sails/blob/master/js/README.md#generate-library-from-idl) (for programs built with the [Sails Library](https://wiki.vara.network/docs/build/sails/)) or metadata (for programs built using the [Gear library](https://wiki.vara.network/docs/build/gstd/)):

### Sails program based provider

```jsx
import { SignlessTransactionsProvider } from 'gear-ez-transactions';
import { useProgram } from '@gear-js/react-hooks';
import { Program } from './lib';

function SignlessTransactionsProvider({ children }: ProviderProps) {
  const { data: program } = useProgram({ library: Program, id: '0x...' });

  return (
    <SignlessTransactionsProvider programId={'0x...'} program={program}>
      {children}
    </SignlessTransactionsProvider>
  );
}
```

### Metadata based provider

```jsx
import { SignlessTransactionsProvider } from 'gear-ez-transactions';

return (
  <SignlessTransactionsProvider programId={'0x...'} metadataSource={metaTxt}>
    {children}
  </SignlessTransactionsProvider>
);
```

### useSignlessTransactions

The package provides a `useSignlessTransactions` hook that returns a context with all required properties:

```jsx
import { useSignlessTransactions } from 'gear-ez-transactions';

const signlessContext = useSignlessTransactions();

const { 
  pair, 
  session, 
  isSessionReady, 
  voucher, 
  isLoading, 
  setIsLoading, 
  isActive, 
  isSessionActive,
  isAutoSignlessEnabled 
} = signlessContext;
```

### Auto Signless

The library provides automatic signless session management through the `isAutoSignlessEnabled` flag. When enabled, signless modals are automatically displayed whenever a transaction needs them, without requiring manual session management.

You can enable auto signless globally via `SignlessTransactionsProvider`:

```jsx
import { SignlessTransactionsProvider } from 'gear-ez-transactions';

return (
  <SignlessTransactionsProvider isAutoSignlessEnabled {...restProps}>
    {children}
  </SignlessTransactionsProvider>
);

```

Or you can override auto-signless settings per transaction:

```ts
const params = await prepareEzTransactionParams({
  isAutoSignlessEnabled: true,
  autoSignless: {
    allowedActions: ['Play', 'Pause'],
    boundSessionDuration: 600000,
  },
});
```

The hook will automatically open the appropriate modal (`CreateSessionModal` or `EnableSessionModal`) when needed.

## Use gasless and signless transaction together

Combined Workflow:

- The frontend generates a sub-account with limited permissions.
- This sub-account then communicates with the backend to request a gas voucher.
- The voucher is applied to the transaction, covering the gas fees.
- The sub-account automatically signs the transaction, completing the process without requiring any manual input from the user.

`EzTransactionsProvider` implements logic that allows the use of gasless and signless transactions together, e.g. disabling gasless when signless is active and requesting a voucher before a signless session is created. It uses both the signless and gasless contexts, so it needs to be wrapped by `GaslessTransactionsProvider` and `SignlessTransactionsProvider`.

```jsx
import { EzTransactionsProvider } from 'gear-ez-transactions';

return <EzTransactionsProvider>{children}</EzTransactionsProvider>;
```

The package provides a `useEzTransactions` hook that returns both gasless and signless contexts:

```jsx
import { useEzTransactions } from 'gear-ez-transactions';

const { gasless, signless } = useEzTransactions();
```

### usePrepareEzTransactionParams

To work with signless and gasless transactions together, sending transactions requires a `sessionForAccount` parameter and using `pair` as the sender's account. Also, the `voucherId` needs to be requested. `usePrepareEzTransactionParams` implements this logic and handles automatic signless session management:

```jsx
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

const { prepareEzTransactionParams } = usePrepareEzTransactionParams({
  isAutoSignlessEnabled: true,
  autoSignless: { allowedActions: ['ActionOne', 'ActionTwo'] },
});

const sendMessage = async () => {
  const params = await prepareEzTransactionParams();
  // Use these parameters to send a message to your program
  const { sessionForAccount, account, voucherId, gasLimit } = params;
};

// You can disable auto signless per call
const paramsWithoutAuto = await prepareEzTransactionParams({ 
  isAutoSignlessEnabled: false 
});

// Or override auto signless settings per call
const paramsWithCustomSettings = await prepareEzTransactionParams({
  autoSignless: {
    allowedActions: ['CustomAction'],
    boundSessionDuration: 300000,
  }
});
```

### UI components

The package provides components for enabling and disabling gasless and signless transactions.

```jsx
import { EzSignlessTransactions, EzGaslessTransactions, EzTransactionsSwitch } from 'gear-ez-transactions';

// Buttons
<EzSignlessTransactions allowedActions={allowedActions} />
<EzGaslessTransactions />

// Switch
<EzTransactionsSwitch allowedActions={allowedActions} />
```

`allowedActions`: `string[]` - A list of actions that the program allows for signless transactions.
