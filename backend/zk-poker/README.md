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

Telegram bot for [ZK Poker application](https://github.com/gear-foundation/dapps/tree/master/frontend/apps/zk-poker).

This bot allows users to start playing ZK Poker directly from Telegram by providing a seamless interface to launch the web application.

## Features

- **Game Launch**: Start ZK Poker game with `/start` command
- **Web Integration**: Redirects users to the ZK Poker web application
- **User Tracking**: Logs user interactions for monitoring

## Getting started

### Install packages:

```sh
yarn install
```

### Declare environment variables:

Create `.env` file, `.env.example` will let you know what variables are expected.

To get a bot token:

1. Contact [@BotFather](https://t.me/botfather) on Telegram
2. Create a new bot with `/newbot` command
3. Copy the provided token to your `.env` file

### Run the bot:

```sh
yarn start
```

## Bot Commands

- `/start` - Launch ZK Poker game

The bot will redirect users to the ZK Poker web application where they can play the game with zero-knowledge proofs for secure card handling.
