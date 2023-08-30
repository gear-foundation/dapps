<p align="center">
  <a href="https://gear-tech.io">
    <img src="https://github.com/gear-tech/gear/blob/master/images/logo-grey.png" width="400" alt="GEAR">
  </a>
</p>
<p align=center>
    <a href="https://github.com/gear-tech/gear-js/blob/master/LICENSE"><img src="https://img.shields.io/badge/License-GPL%203.0-success"></a>
</p>
<hr>

## Description

An example of React application that demonstrates how to work with a [Non-Fungible Token](https://wiki.gear-tech.io/examples/gnft-721/). The Rust implementation of NFT smart-contract is available on [GitHub](https://github.com/gear-dapps/non-fungible-token).

The application implements the ability to mint NFTs, view all NFTs minted by any account in the contract, as well as view NFTs that someone has approved to the current account (AprovedToMe) with the possibility of further transfer to another account.

## Getting started

Download this repository.

### Install packages:

```sh
yarn install
```

### Declare environment variables:

Create `.env` file, `.env.example` will let you know what variables are expected.

In order for all features to work as expected, the node and it's runtime version should be chosen based on the current `@gear-js/api` version.

In case of issues with the application, try to switch to another network or run your own local node and specify its address in the `.env` file. When applicable, make sure the smart contract(s) wasm files are uploaded and running in this network accordingly.

### Run the app:

```sh
yarn start
```
