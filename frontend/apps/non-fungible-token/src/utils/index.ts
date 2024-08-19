import { ADDRESS } from 'consts';
import { getMintDetails, getMintPayload } from './form';

const getIpfsAddress = (cid: string) => `${ADDRESS.IPFS_GATEWAY}/${cid}`;

export { getIpfsAddress, getMintDetails, getMintPayload };
