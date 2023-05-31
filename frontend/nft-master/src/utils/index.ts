import { ADDRESS } from 'consts';

const copyToClipboard = (value: string) => navigator.clipboard.writeText(value).then(() => console.log('Copied!'));

const getIpfsAddress = (cid: string) => `${ADDRESS.IPFS_GATEWAY}/${cid}`;

export { copyToClipboard, getIpfsAddress };
