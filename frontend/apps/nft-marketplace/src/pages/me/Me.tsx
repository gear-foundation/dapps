import { List } from '@/components';
import { useMergedOwnerNFTs } from '@/hooks';

function Me() {
  const { ownerNFTs, isEachNFTRead } = useMergedOwnerNFTs();

  return (
    <List
      heading="My NFTs"
      NFTs={{ list: ownerNFTs, isRead: isEachNFTRead, fallback: "You don't have any tokens yet." }}
    />
  );
}

export { Me };
