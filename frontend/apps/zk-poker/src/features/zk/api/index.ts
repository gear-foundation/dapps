import { ENV } from '@/app/consts';

import { ZkResultRequest, ZkResultResponse, ZkTaskApiResponse, ZkTaskResponse } from './types';

const getZkTask = async (lobbyAddress?: string, playerAddress?: string): Promise<ZkTaskResponse> => {
  if (!lobbyAddress || !playerAddress) throw new Error('Lobby address or player address is not defined');

  try {
    const res = await fetch(
      `${ENV.ZK_POKER_BACKEND}/api/poker/task?lobbyAddress=${lobbyAddress}&playerAddress=${playerAddress}`,
    );

    if (!res.ok) throw new Error('Failed to fetch zk task');

    const proofData = (await res.json()) as ZkTaskApiResponse;

    // ! TODO: check is it fixed on backend (200 response code on error)
    if ('message' in proofData) {
      throw new Error(proofData.message);
    }

    return proofData;
  } catch (error) {
    console.error(error);
    throw new Error(error instanceof Error ? error.message : 'Failed to fetch proof data');
  }
};

const postZkResult = async (payload: ZkResultRequest): Promise<ZkResultResponse> => {
  try {
    const res = await fetch(`${ENV.ZK_POKER_BACKEND}/api/poker/result`, {
      method: 'POST',
      body: JSON.stringify(payload),
      headers: {
        'Content-Type': 'application/json',
      },
    });
    if (!res.ok) throw new Error('Failed to post zk result');
    const result = (await res.json()) as ZkResultResponse;
    return result;
  } catch (error) {
    console.error(error);
    throw new Error('Failed to post zk result');
  }
};

export { getZkTask, postZkResult };
