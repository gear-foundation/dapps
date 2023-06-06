import { isHex } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';
import { ADDRESS } from 'consts';

const copyToClipboard = (value: string) => navigator.clipboard.writeText(value).then(() => console.log('Copied!'));

const getIpfsAddress = (cid: string) => `${ADDRESS.IPFS_GATEWAY}/${cid}`;

const isProgramIdValid = (value: string): value is HexString => isHex(value, 256);

export { copyToClipboard, getIpfsAddress, isProgramIdValid };
