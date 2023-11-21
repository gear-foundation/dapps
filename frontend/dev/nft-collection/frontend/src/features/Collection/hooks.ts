import { useEffect, useMemo } from 'react';
import { useAtom } from 'jotai';
import { HexString, MessagesDispatched } from '@gear-js/api';
import { useAlert, useApi, useSendMessage } from '@gear-js/react-hooks';
import { UnsubscribePromise } from '@polkadot/api/types';
import { useMetadata, useProgramMetadata } from '@/hooks';
import { ADDRESS } from '@/consts';
import factoryMetaTxt from '@/assets/meta/collection_factory.meta.txt';
import collectionMetaTxt from '@/assets/meta/nft_collection.meta.txt';
import { COLLECTIONS, COLLECTION_CONTRACTS } from '@/features/Collection/atoms';
import { Collection } from './types';

function useFactoryMetadata() {
  const meta = useMetadata(factoryMetaTxt);

  const memoizedMeta = useMemo(() => meta, [meta]);

  return memoizedMeta;
}

function useFactoryMessage() {
  const meta = useFactoryMetadata();

  const message = useSendMessage(ADDRESS.FACTORY, meta);

  return { meta, message };
}

const handleStateChange = ({ data }: MessagesDispatched, programId: HexString, onChange: () => void) => {
  const changedIDs = data.stateChanges.toHuman() as HexString[];
  const isAnyChange = changedIDs.some((id) => id === programId);

  if (isAnyChange) {
    onChange();
  }
};

function useCollectionsState() {
  const { api, isApiReady } = useApi();
  const alert = useAlert();

  const masterContractAddress = ADDRESS.FACTORY;
  const masterMetadata = useProgramMetadata(factoryMetaTxt);
  const metadata = useProgramMetadata(collectionMetaTxt);

  const [collectionContracts, setCollectionContracts] = useAtom(COLLECTION_CONTRACTS);
  const [collections, setCollections] = useAtom(COLLECTIONS);

  const readMasterContractState = () => {
    if (!isApiReady || !masterContractAddress || !masterMetadata) {
      return;
    }

    const programId = masterContractAddress;

    api.programState
      .read({ programId }, masterMetadata)
      .then((codec) => codec.toHuman() as any)
      .then(({ ownerToCollection }) => setCollectionContracts(ownerToCollection))
      .catch(({ message }: Error) => alert.error(message));
  };

  useEffect(() => {
    setCollectionContracts([]);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [masterContractAddress]);

  useEffect(() => {
    if (!isApiReady || !masterContractAddress || !masterMetadata) return;

    readMasterContractState();

    const unsub = api.gearEvents.subscribeToGearEvent('MessagesDispatched', (event) =>
      handleStateChange(event, masterContractAddress, readMasterContractState),
    );

    return () => {
      unsub.then((unsubCallback) => unsubCallback());
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, masterContractAddress, masterMetadata]);

  const readNFTContractsStates = () => {
    if (!collectionContracts.length || !metadata) {
      return;
    }

    const promises = collectionContracts.map(([ownerAddress, { address, timeCreation }]) =>
      api.programState
        .read({ programId: address }, metadata)
        .then((codec) => ({ ...(codec.toHuman() as any), id: address, timeCreation })),
    );

    Promise.all(promises)
      .then((result) => {
        setCollections(
          result.reduce(
            (acc, collection: Collection) => ({
              ...acc,
              [collection.id]: {
                ...collection,
                tokens: collection.tokens.map((token, i) => ({
                  ...token,
                  name: `NFT on Vara Incentivized Testnet - ${i + 1}`,
                  description: `This one-of-a-kind NFT is more than just collectible; 
                  this is a token of ownership, enabling you to hold a piece of this dynamic 
                  digital tapestry and become part of its legacy. 
                  Through smart contracts on Vara Incentivized Testnet, 
                  we've ensured that each transaction is transparent, 
                  secure, and eco-friendly.`,
                  collectionName: collection.collection.name,
                  timeMinted: token.owner ? token.timeMinted : collection.timeCreation,
                  id: `${collection.id}/${i}`,
                })),
              },
            }),
            {},
          ),
        );
      })
      .catch(({ message }: Error) => {
        alert.error(message);
        console.log(message);
      });
  };

  useEffect(() => {
    if (!collectionContracts.length) {
      return;
    }

    readNFTContractsStates();

    const unsubs: UnsubscribePromise[] = [];

    // TODO: if state of any contract changes,
    // we reread every single one specified in master contract.
    // need to find the way to optimize
    collectionContracts.forEach(([programId]) => {
      const unsub = api.gearEvents.subscribeToGearEvent('MessagesDispatched', (event) =>
        handleStateChange(event, programId, readNFTContractsStates),
      );

      unsubs.push(unsub);
    });

    return () => {
      unsubs.map((unsub) => unsub.then((unsubCallback) => unsubCallback()));
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [collectionContracts]);

  return { isCollectionsRead: !!Object.keys(collections).length };
}

export { useFactoryMessage, useCollectionsState };
