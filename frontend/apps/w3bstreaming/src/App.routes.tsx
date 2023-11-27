export const STREAMS = 'streams';
export const CREATE_STREAM = 'create-stream';
export const ACCOUNT = 'account';
export const STREAM = 'stream/:id';

export const routes: { [key: string]: { url: string } } = {
  Streams: {
    url: `${STREAMS}`,
  },
  'Create Stream': {
    url: `${CREATE_STREAM}`,
  },
  Account: {
    url: `${ACCOUNT}`,
  },
};
