import { useAtomValue } from 'jotai';
import { useLocation } from 'react-router';
import { useEffect, useState } from 'react';
import { GalleryCollection } from '@/features/Collection/components/GalleryCollection';
import { AllData } from '@/features/Collection/types';

import { COLLECTIONS } from '@/features/Collection/atoms';
import { CollectionPreview } from '@/features/Collection/components/CollectionPreview';
import { makeNftsStructure } from '../ExplorePage/utils';
import { NftPreview } from '@/features/Nft/components/NftPreview';
import { COLLECTION, NFT } from '@/routes';
import { Link } from '@/ui';

function SearchPage() {
  const location = useLocation();
  const collections = useAtomValue(COLLECTIONS);

  const [allData, setAllData] = useState<AllData>({
    collections: [],
    nfts: [],
  });
  const [chosenData, setChosenData] = useState<'nfts' | 'collections'>('collections');

  useEffect(() => {
    const query = new URLSearchParams(location.search).get('query');
    const collectionKeys = Object.keys(collections);

    if (query) {
      setAllData({
        collections: collectionKeys
          .filter((key) => collections[key].collection.name.toLowerCase().includes(query.toLowerCase()))
          .map((key) => ({
            id: collections[key].id,
            component: (
              <Link to={`${COLLECTION}/${collections[key].id}`}>
                <CollectionPreview collection={collections[key]} />
              </Link>
            ),
          })),
        nfts: makeNftsStructure(collections)
          .filter((token) => token.name.toLowerCase().includes(query.toLowerCase()))
          .map((token) => ({
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
            id: token.id,
          })),
      });
    }
  }, [location.search, collections]);

  const handleChooseData = (option: 'nfts' | 'collections') => {
    setChosenData(option);
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

  return (
    <GalleryCollection
      title={`Search results for: ${new URLSearchParams(location.search).get('query')}`}
      switchMenu={switchOptions}
      data={allData[chosenData]}
    />
  );
}

export { SearchPage };
