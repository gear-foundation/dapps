import { GearApi, GearKeyring, getWasmMetadata, CreateType, Codec } from '@gear-js/api';
import { readFileSync } from 'fs';
import { payloads } from './payloads';

export async function deploy(name: string, symbol: string, base_uri: string) {
    const gearApi = await GearApi.create();
    const jsonKeyring = readFileSync(process.env.PATH_TO_KEYS).toString();
    const account = GearKeyring.fromJson(jsonKeyring, process.env.PASSWORD);
    const code = readFileSync(process.env.OPT_WASM);
    const metaFile = readFileSync(process.env.META_WASM);
    const meta =  await getWasmMetadata(metaFile);

    let initMTK = payloads.init(name, symbol, base_uri);
    const gas = await gearApi.program.gasSpent.init(
        `0x${account.address}`,
        code,
        initMTK,
        0,
        meta
    );
    console.log("GAS SPENT", gas.toNumber());

    const program = gearApi.program.submit({ code, initPayload: initMTK, gasLimit: gas }, meta);
    await gearApi.program.signAndSend(account, (data) => {
        console.log(data.toHuman());
    });
    console.log("Program was initialized with id", program.programId);
}

export async function send(payload: any, destination:  Buffer | `0x${string}`) {
    const gearApi = await GearApi.create();
    const account = await GearKeyring.fromMnemonic(process.env.MNEMONIC);

    const metaFile = readFileSync(process.env.META_WASM);
    const meta =  await getWasmMetadata(metaFile);
    console.log(account);

    const gas = await gearApi.program.gasSpent.handle(
        `0x${account.address}`,
        destination,
        payload,
        10010,
        meta,
    );
    console.log('GAS SPENT', gas.toHuman());

    try {
        const message = {
            destination: destination.toString(),
            payload,
            gasLimit: gas,
            value: 10010
        };
        await gearApi.message.submit(message, meta);
    } catch (error) {
    console.error(`${error.name}: ${error.message}`);
    }
    try {
    await gearApi.message.signAndSend(account, (event) => {
        console.log(event.toHuman());
    });
    } catch (error) {
    console.error(`${error.name}: ${error.message}`);
    }
}

export async function subscribe() {
    const gearApi = await GearApi.create();

    const metaFile = process.env.META_WASM ? readFileSync(process.env.META_WASM) : undefined;
    const meta = metaFile ? await getWasmMetadata(metaFile) : undefined;

    gearApi.gearEvents.subscribeToLogEvents(({ data: { id, source, payload, reply } }) => {
        console.log(`
          Log:
          messageId: ${id.toHex()}
          from program: ${source.toHex()}
        payload: ${
           payload.toHuman()
            }
        ${
          reply.isSome
            ? `reply to: ${reply.unwrap()[0].toHex()}
          with error: ${reply.unwrap()[1].toNumber() === 0 ? false : true}
          `
            : ''
        }
        `);

        try {
          console.log(CreateType.create(meta.handle_output, payload, meta).toHuman())
        } catch (error) {
          console.log(error);
        }
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
}

export async function read_state(): Promise<Codec>{
    const gearApi = await GearApi.create();
    const metaWasm = readFileSync(process.env.META_WASM);
    const current_state = await gearApi.programState.read(process.env.PROGRAM_ID, metaWasm, { CurrentState: null });
    return current_state;
}