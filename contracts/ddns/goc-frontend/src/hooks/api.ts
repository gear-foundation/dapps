import { Hex } from '@gear-js/api';
import { useMetadata, useSendMessage } from '@gear-js/react-hooks';
import { Lottery } from 'types';
import metaWasm from 'assets/wasm/game_of_chance.meta.wasm';
import { useReadState } from './state';

const programId = process.env.REACT_APP_PROGRAM_ID as Hex;
const metaBufferBase64 = process.env.REACT_APP_META_BUFFER_BASE_64 as string;
const metaTypes = process.env.REACT_APP_META_TYPES as Hex;

const metadata = {
  types: metaTypes,
  init_input: 'GOCInit',
  init_output: '',
  async_init_input: '',
  async_init_output: '',
  handle_input: 'GOCAction',
  handle_output: 'GOCEvent',
  async_handle_input: '',
  async_handle_output: '',
  title: 'Game of chance',
  meta_state_input: '',
  meta_state_output: 'GOCState',
};

const metaBuffer = Buffer.from(metaBufferBase64, 'base64');

function useLottery() {
  const { state, isStateRead } = useReadState<Lottery>(programId, metaBuffer);

  // const { metadata: metaLog, metaBuffer: bufferLog } = useMetadata(metaWasm);
  // console.log('metaBuffer: ', metaLog);
  // console.log('metadata: ', bufferLog?.toString('base64'));

  return { lottery: state, isLotteryRead: isStateRead };
}

function useLotteryMessage() {
  return useSendMessage(programId, metadata);
}

export { useLottery, useLotteryMessage };
