import { GearApi, GearKeyring, getWasmMetadata, Hex } from '@gear-js/api';
import { web3Enable, web3Accounts } from '@polkadot/extension-dapp';
import { deploy, makeMove, Move, reveal } from 'rock-paper-scissors-api';

var programId: Hex;

async function getAccount() {
  await web3Enable('Gear App');
  const account = (await web3Accounts())[2];
  return {
    ...account,
    decodedAddress: GearKeyring.decodeAddress(account.address),
    balance: { 
      value: '0', 
      unit: 'sd',
    },
  }
}

export async function first() {
    const gearApi = await GearApi.create();
    const account = await getAccount();

    deploy(
      gearApi,
      account,
      0,
      [account.decodedAddress],
      function(id) {
        programId = id;
      },
    )
}

export async function second() {
  const gearApi = await GearApi.create();
  const account = await getAccount();

  console.log(Move.LIZARD.toString() + '123')
  makeMove(
    gearApi,
    programId,
    account,
    Move.LIZARD,
    '123',
    0,
    function(event) {
      console.log(event.toHuman());
    },
  )
}

export async function third() {
  const gearApi = await GearApi.create();
  const account = await getAccount();

  reveal(
    gearApi,
    programId,
    account,
    Move.LIZARD,
    '123',
    function(event) {
      console.log(event.toHuman());
    },
  )
}