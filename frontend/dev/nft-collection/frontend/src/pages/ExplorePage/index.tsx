import { useEffect, useState } from 'react';
import { useAtomValue } from 'jotai';
import { COLLECTIONS } from '@/features/Collection/atoms';
import { CollectionPreview } from '@/features/Collection/components/CollectionPreview';
import { GalleryCollection } from '@/features/Collection/components/GalleryCollection';
import { Link } from '@/ui';
import { COLLECTION, NFT } from '@/routes';
import { AllData } from '@/features/Collection/types';
import { ACCOUNT_ATOM } from '@/atoms';
import { NftPreview } from '@/features/Nft/components/NftPreview';
import { CollectionFilter, NftFilter, makeCollectionsStructure, makeNftsStructure } from './utils';

function ExplorePage() {
  const collections = useAtomValue(COLLECTIONS);
  const [allData, setAllData] = useState<AllData>({
    collections: [],
    nfts: [],
  });
  const [chosenData, setChosenData] = useState<'nfts' | 'collections'>('collections');
  const [chosenFilter, setChosenFilter] = useState<'availableToMint' | 'allCollections'>('availableToMint');
  const account = useAtomValue(ACCOUNT_ATOM);

  const handleChooseData = (option: 'nfts' | 'collections') => {
    setChosenData(option);
  };

  const handleChooseFilter = (option: 'availableToMint' | 'allCollections') => {
    setChosenFilter(option);
  };

  const filterOptions = {
    'Available to Mint': {
      label: 'Available to Mint',
      value: 'availableToMint',
      onSelect: () => handleChooseFilter('availableToMint'),
    },
    'All Collections': {
      label: 'All Collections',
      value: 'allCollections',
      onSelect: () => handleChooseFilter('allCollections'),
    },
  };

  const switchOptions = [
    {
      name: 'NFTs',
      value: 'nfts',
      onSelect: () => handleChooseData('nfts'),
    },
    {
      name: 'Collections',
      value: 'collections',
      onSelect: () => handleChooseData('collections'),
      activeByDefault: true,
    },
  ];

  useEffect(() => {
    if (collections) {
      setAllData(() => ({
        collections: new CollectionFilter(makeCollectionsStructure(collections))
          .filter(chosenFilter)
          .map((collection) => ({
            component: (
              <Link to={`${COLLECTION}/${collection.id}`}>
                <CollectionPreview collection={collection} />
              </Link>
            ),
            id: collection.id,
          })),
        nfts: new NftFilter(makeNftsStructure(collections)).filter(chosenFilter).map((token) => ({
          component: (
            <Link to={`${NFT}/${token.id}`}>
              <NftPreview
                url={token.medium}
                name={token.name}
                collectionName={token.collectionName}
                owner={token.owner}
                timeMinted={token.timeMinted}
              />
            </Link>
          ),
          id: `${token.timeMinted}-${token.medium}-${token.owner}`,
        })),
      }));
    }
  }, [account?.decodedAddress, collections, chosenFilter]);

  return (
    <GalleryCollection
      title="Explore NFTs & Collections"
      switchMenu={switchOptions}
      filterOptions={filterOptions}
      data={allData[chosenData]}
    />
  );
}

export { ExplorePage };
