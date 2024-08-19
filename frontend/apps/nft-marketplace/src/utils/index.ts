import { ADDRESS, LOCAL_STORAGE } from 'consts';
import { getMintDetails, getMintPayload } from './form';
import { getAuctionDate, getListingProps, getNFTProps } from './nft';

const isHex = (value: unknown) => {
  const hexRegex = /^0x[\da-fA-F]+/;

  return typeof value === 'string' && hexRegex.test(value);
};

const getIpfsAddress = (cid: string) => `${ADDRESS.IPFS_GATEWAY}/${cid}`;

const getMilliseconds = (value: string) => Number(value) * 60000;

export {
  isHex,
  getMintDetails,
  getMintPayload,
  getIpfsAddress,
  getNFTProps,
  getAuctionDate,
  getListingProps,
  getMilliseconds,
};
