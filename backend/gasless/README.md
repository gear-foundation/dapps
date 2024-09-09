<p align="center">
  <a href="https://gear-tech.io">
    <img src="https://github.com/gear-tech/gear/blob/master/images/logo-grey.png" width="400" alt="GEAR">
  </a>
</p>
<p align=center>
    <a href="https://github.com/gear-tech/gear-js/blob/master/LICENSE"><img src="https://img.shields.io/badge/License-GPL%203.0-success"></a>
</p>
<hr>

# Gasless

## Description

Example of backend for issuing and revoking vouchers (gasless).

Example contains three functions for issuing and revoking vouchers:

1. `issue(account: HexString, programId: HexString, amount: number, durationInSec: number): Promise<string>`
   - Issues a voucher for the given account, programId, amount, and duration.
   - Parameters:
     - `account`: The account to issue the voucher for.
     - `programId`: The programId to issue the voucher for.
     - `amount`: The amount to issue the voucher for.
     - `durationInSec`: The duration to issue the voucher for in seconds.
   - Returns: A Promise that resolves to the voucherId as a string.

2. `prolong(voucherId: HexString, account: string, balance: number, prolongDurationInSec: number): Promise<void>`
   - Prolongs the voucher with the given voucherId, account, balance, and prolongDuration.
   - Parameters:
     - `voucherId`: The voucherId to prolong.
     - `account`: The account to prolong the voucher for.
     - `balance`: The required balance to top up the voucher.
     - `prolongDurationInSec`: The duration to prolong the voucher for in seconds.
   - Returns: A Promise that resolves when the operation is complete.

3. `revoke(voucherId: HexString, account: string): Promise<void>`
   - Revokes the voucher with the given voucherId and account.
   - Parameters:
     - `voucherId`: The voucherId to revoke.
     - `account`: The account to revoke the voucher for.
   - Returns: A Promise that resolves when the operation is complete.

These functions are part of the `GaslessService` class, which interacts with the Gear API to manage vouchers.


## Getting started

### Install packages:

```sh
yarn install
```

### Declare environment variables:

Create `.env` file, `.env.example` will let you know what variables are expected.


### Build the app:

```sh
yarn build
```

### Run the app:

```sh
yarn start
```
