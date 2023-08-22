import { HexString } from '@polkadot/util/types';
import { Item, Token } from 'types';

const isEmptyHex = (value: HexString) => value.startsWith('0x00');

const getAddress = (value: HexString) => (isEmptyHex(value) ? ('' as HexString) : value);

const getItemData = (item: Item, nft: Token) => {
  const { state, by } = item.state;
  const { name, description } = nft;

  const producer = getAddress(item.producer);
  const distributor = getAddress(item.distributor);
  const retailer = getAddress(item.retailer);

  return { name, description, state, by, producer, distributor, retailer };
};

export { getItemData };
