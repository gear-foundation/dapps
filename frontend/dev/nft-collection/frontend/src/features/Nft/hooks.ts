import { decodeAddress } from '@gear-js/api';
import { useMemo } from 'react';
import { useSearchParams } from 'react-router-dom';

export function useNFTSearch() {
  const [searchParams, setSearchParams] = useSearchParams();
  const searchQuery = searchParams.get('query') || '';

  const decodedQueryAddress = useMemo(() => {
    if (!searchQuery) return;

    try {
      return decodeAddress(searchQuery);
    } catch (error) {
      return undefined;
    }
  }, [searchQuery]);

  const resetSearchQuery = () => {
    searchParams.delete('query');

    setSearchParams(searchParams);
  };

  return { searchQuery, decodedQueryAddress, resetSearchQuery };
}

export const useNFTs = (): any => {};
