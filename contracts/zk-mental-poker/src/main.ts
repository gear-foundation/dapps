import { GearApi, GearKeyring, decodeAddress } from "@gear-js/api";
import { SailsProgram } from "./programs/poker.js";   
import { PtsProgram } from "./programs/pts.js";     

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

import { F1Field } from "ffjavascript";
import * as snarkjs from "snarkjs";
import { groth16 } from "snarkjs";

import { initDeck, keyGen, projectiveAdd } from "zk-shuffle-proof";
import { shuffleDeckWithProofs } from "./game/shuffle.js";
import { dealHands } from "./game/deal.js";
import { partialDecrypt, finalDecryptAndShow } from "./game/decrypt.js";
import { ecPointToNumberArrays } from "./utils/ec.js";
import { q, a, d, basePoint } from "./config.js";


function getPkZk(pub: { X: bigint; Y: bigint; Z: bigint }) {
  const { x, y, z } = ecPointToNumberArrays(pub);
  return { x, y, z }; 
}

async function loadAccounts(paths: string[], pass: string) {
  return paths.map(p => GearKeyring.fromJson(readFileSync(resolve(p), "utf-8"), pass));
}

async function main() {
  const api = await GearApi.create({ providerAddress: "wss://testnet.vara.network" });
  const encryptWasmFile = resolve("./build/shuffle_encrypt.wasm");
  const encryptZkeyFile = resolve("./build/shuffle_encrypt.zkey");
  const encryptVkey = await snarkjs.zKey.exportVerificationKey(
    new Uint8Array(Buffer.from(readFileSync(encryptZkeyFile)))
  );
  // create 3 accounts
  const accountFiles = ["./accounts/account1.json", "./accounts/account2.json", "./accounts/account3.json"];
  const keyrings = await loadAccounts(accountFiles, "123456");
  const numPlayers = 3, numCards = 52, numBits = 64;

  const F = new F1Field(q);
  const players = Array.from({ length: numPlayers }, () => keyGen(numBits));
  const playerPks = players.map(p => getPkZk(p.pk)); // для контракта

  // deploy PTS
  const ptsProgram = new PtsProgram(api);
  const ptsCode = readFileSync(resolve("./contracts/target/wasm32-gear/release/pts.opt.wasm"));
  const ptsCtor = await ptsProgram.newCtorFromCode(ptsCode, 10000, 10000).withAccount(keyrings[0]).calculateGas();
  await ptsCtor.withGas(200000000000n).signAndSend();
  console.log(`\nPTS deployed: ${ptsProgram.programId}`);

  // deploy Poker
  const pokerProgram = new SailsProgram(api);
  const gameConfig = {
    admin_id: decodeAddress(keyrings[0].address),
    admin_name: "Alice",
    lobby_name: "Lobby",
    small_blind: 10,
    big_blind: 100,
    starting_bank: 100,
    time_per_move_ms: 1_000_000
  };
  const sessionConfig = {
    gas_to_delete_session: 20_000_000_000,
    minimum_session_duration_ms: 10_000,
    ms_per_block: 3
  };
  const zkProgramId = "0x7e2826b2b6747324efc1b2b63ae8ba144f74b6af0d1d8dbaa65e3a1e0b4f0d5d"; // твой ID
  const pokerCode = readFileSync(resolve("./contracts/target/wasm32-gear/release/poker.opt.wasm"));
  const pokerCtor = await pokerProgram
    .newCtorFromCode(pokerCode, gameConfig, sessionConfig, ptsProgram.programId, playerPks[0], null, zkProgramId)
    .withAccount(keyrings[0]).calculateGas();
  await pokerCtor.withGas(200000000000n).signAndSend();
  console.log(`\nPoker deployed: ${pokerProgram.programId}`);

  const addAdminB = await ptsProgram.pts.addAdmin(pokerProgram.programId).withAccount(keyrings[0]).calculateGas();
  const addAdminR = (await addAdminB.withGas(200000000000n).signAndSend()).response;
  console.log(`\nAdd admin message sent.`);
  console.log(`\nPTS replied: \n\t${JSON.stringify(await addAdminR())}`);

  for (let i=1;i<numPlayers;i++) {
    const accuralB = await ptsProgram.pts.getAccural().withAccount(keyrings[i]).calculateGas();
    const accuralR = (await accuralB.withGas(200000000000n).signAndSend()).response;
    console.log(`\nGet accural message sent.`);
    console.log(`\nPTS replied: \n\t${JSON.stringify(await accuralR())}`);

    const regB = await pokerProgram.poker.register("Player", playerPks[i], null).withAccount(keyrings[i]).calculateGas();
    const regR = (await regB.withGas(200000000000n).signAndSend()).response;
    console.log(`\nRegister message sent.`);
    console.log(`\nPoker replied: \n\t${JSON.stringify(await regR())}`);
  }

  {
    const b = await pokerProgram.poker.startGame(null).withAccount(keyrings[0]).calculateGas();
    const r = (await b.withGas(200000000000n).signAndSend()).response;
    console.log(`\nStart game message sent.\n`);
    console.log(`\nProgram replied: \n\t${JSON.stringify(await r())}`);
  }

  const aggKey = players.reduce(
    (acc, p) => projectiveAdd(F, a, d, acc, p.pk),
    { X: 0n, Y: 1n, Z: 1n }
  );
  let deck: bigint[][] = initDeck(numCards);

  const { deck: shuffledDeck, cardMap } = await shuffleDeckWithProofs({
    players, numCards, F, a, d, base: basePoint, aggKey, deck,
    encryptWasmFile, encryptZkeyFile, encryptVkey,
    groth16, snarkjs, program: pokerProgram, adminKeyring: keyrings[0]
  });

  const hands = dealHands(shuffledDeck, numPlayers);

  await partialDecrypt({
    program: pokerProgram, players, playerHands: hands,
    F, a, d, base: basePoint, keyrings
  });

  await finalDecryptAndShow({
    program: pokerProgram, players, keyrings,
    F, a, d, cardMap
  });

  console.log("\nFlow complete\n");
}

await main();
