const { GearApi } = require("@gear-js/api");

const events = async () => {
    const gearApi = await GearApi.create();

  gearApi.gearEvents.subscribeToLogEvents(({ data: { id, source, payload, reply } }) => {
    console.log(`
    Log:
      messageId: ${id.toHex()}
      from program: ${source.toHex()}
      payload: ${payload.toJSON()}
    ${
      reply.isSome
        ? `reply to: ${reply.unwrap()[0].toHex()}
      with error: ${reply.unwrap()[1].toNumber() === 0 ? false : true}
      `
        : ''
    }
    `);
  });

  gearApi.gearEvents.subscribeToProgramEvents(({ method, data: { info, reason } }) => {
    console.log(`
      ${method}:
      programId: ${info.programId.toHex()}
      initMessageId: ${info.messageId.toHex()}
      origin: ${info.origin.toHex()}
      ${reason ? `reason: ${reason.toHuman()}` : ''}
      `);
  });

};

async function main() {
    await events();
}

main();
