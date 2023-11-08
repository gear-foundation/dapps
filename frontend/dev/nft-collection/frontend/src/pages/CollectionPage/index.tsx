import { useAtomValue } from 'jotai';
import { useParams } from 'react-router';
import { COLLECTIONS } from '@/features/Collection/atoms';
import { Collection } from '@/features/Collection/components/Collection';

function CollectionPage() {
  const { id } = useParams();
  const collections = useAtomValue(COLLECTIONS);

  return id && <Collection data={collections[id]} />;
}

export { CollectionPage };
