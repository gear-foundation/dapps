export async function startGame(program: any, adminKeyring: any) {
  const builder = await program.poker.startGame(null).withAccount(adminKeyring).calculateGas();
  const response = (await builder.withGas(200000000000n).signAndSend()).response;
  console.log("\nStart game message sent.\n");
  const reply = await response();
  console.log(`\nProgram replied: \n\t${JSON.stringify(reply)}`);
  return reply;
}
