import { CID } from 'ipfs-http-client';

const getMintDetails = (attributesValue?: { key: string; value: string }[], rarity?: string) => {
  const attributes = attributesValue?.reduce((accumulator, { key, value }) => ({ ...accumulator, [key]: value }), {});

  return JSON.stringify({ attributes, rarity });
};

const getMintPayload = (name: string, description: string, imgCid: CID, detailsCid?: CID) => {
  const tokenMetadata = {
    name,
    description,
    media: imgCid.toString(),
    reference: detailsCid ? detailsCid.toString() : '',
  };

  return { Mint: { tokenMetadata, transaction_id: Math.floor(Math.random() * 1000) } };
};

export { getMintDetails, getMintPayload };
