import { ENV } from '@/consts';

import { getMintDetails, getMintPayload } from './form';

const getIpfsAddress = (cid: string) => `${ENV.IPFS_GATEWAY}/${cid}`;

export { getIpfsAddress, getMintDetails, getMintPayload };
