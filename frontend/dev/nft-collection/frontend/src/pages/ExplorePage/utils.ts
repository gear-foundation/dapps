import { Collection, Collections, Token } from '@/features/Collection/types';

export const makeCollectionsStructure = (collections: Collections): Collection[] => {
  const collectionKeys = Object.keys(collections);

  return collectionKeys.map((key) => collections[key]);
};

export const makeNftsStructure = (collections: Collections): Token[] => {
  const collectionKeys = Object.keys(collections);

  return [...collectionKeys.map((key) => collections[key].tokens.map((token) => token))].flat();
};

interface Filter {
  items: Collection[] | Token[];
  filter(param: string): Collection[] | Token[];
}

export class CollectionFilter implements Filter {
  items: Collection[] = [];

  constructor(collections: Collection[]) {
    this.items = collections;
  }

  filter(param: string) {
    if (param === 'availableToMint') {
      return this.items.filter((collection) => collection.tokens.some((token) => !token.owner));
    }

    return this.items;
  }
}

export class NftFilter implements Filter {
  items: Token[] = [];

  constructor(collections: Token[]) {
    this.items = collections;
  }

  filter(param: string) {
    if (param === 'availableToMint') {
      return this.items.filter((token) => !token.owner);
    }

    return this.items;
  }
}
