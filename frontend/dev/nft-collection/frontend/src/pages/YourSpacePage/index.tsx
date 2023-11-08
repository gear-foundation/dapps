import { useEffect, useState } from 'react';
import { useAtomValue } from 'jotai';
import { COLLECTIONS } from '@/features/Collection/atoms';
import { GalleryCollection } from '@/features/Collection/components/GalleryCollection';
import { NftPreview } from '@/features/Nft/components/NftPreview';
import { Button, Link } from '@/ui';

import { ACCOUNT_ATOM } from '@/atoms';
import { OwnerData } from '@/features/Collection/types';
import { CollectionPreview } from '@/features/Collection/components/CollectionPreview';
import { COLLECTION, NFT } from '@/routes';

function YourSpacePage() {
  const collections = useAtomValue(COLLECTIONS);
  const [ownerData, setData] = useState<OwnerData>({
    collections: [],
    nfts: [],
  });
  const [chosenData, setChosenData] = useState<'nfts' | 'collections'>('collections');
  const account = useAtomValue(ACCOUNT_ATOM);

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

  useEffect(() => {
    if (collections) {
      const collectionKeys = Object.keys(collections);

      setData(() => ({
        collections: collectionKeys
          .filter((key) => collections[key].owner === account?.decodedAddress)
          .map((key) => collections[key])
          .map((collection) => ({
            component: (
              <Link to={`${COLLECTION}/${collection.id}`}>
                <CollectionPreview collection={collection} />
              </Link>
            ),
            id: collection.id,
          })),
        nfts: [
          ...collectionKeys.map((key) =>
            collections[key].tokens.filter((token) => token.owner === account?.decodedAddress),
          ),
          ...collectionKeys
            .filter((key) => collections[key].owner === account?.decodedAddress)
            .map((key) => collections[key].tokens.filter((token) => !token.owner).map((token) => token)),
        ]
          .flat()
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
            id: `${token.timeMinted}-${token.medium}-${token.owner}`,
          })),
      }));
    }
  }, [account?.decodedAddress, collections]);

  return (
    <GalleryCollection
      title="Your Space"
      data={ownerData[chosenData]}
      switchMenu={switchOptions}
      emptyText={
        <>
          <span>Create a new collection</span>
          <span>or suggest to specify custom contract address, switch to another network</span>
          <Button label="Create Collection" variant="primary" />
        </>
      }
    />
  );
}

export { YourSpacePage };
