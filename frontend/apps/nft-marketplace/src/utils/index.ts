import { ADDRESS } from 'consts';
import { getMintDetails } from './form';
import { getAuctionDate, getListingProps, getNFTProps } from './nft';

const isHex = (value: unknown) => {
  const hexRegex = /^0x[\da-fA-F]+/;

  return typeof value === 'string' && hexRegex.test(value);
};

const getIpfsAddress = (cid: string) => `${ADDRESS.IPFS_GATEWAY}/${cid.split('://')[1]}`;

const uploadToIpfs = async (files: (File | Blob)[]) => {
  const formData = new FormData();
  files.forEach((file) => formData.append('file', file));

  const response = await fetch(ADDRESS.IPFS, { method: 'POST', body: formData });
  if (!response.ok) throw new Error(response.statusText);

  const result = await (response.json() as Promise<Record<'ipfsHash', string>[]>);
  return result.map(({ ipfsHash }) => `ipfs://${ipfsHash}`);
};

const getMilliseconds = (value: string) => Number(value) * 60000;

export {
  isHex,
  getMintDetails,
  getIpfsAddress,
  uploadToIpfs,
  getNFTProps,
  getAuctionDate,
  getListingProps,
  getMilliseconds,
};
