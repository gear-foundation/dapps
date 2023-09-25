import { useNavigate, useSearchParams, useLocation } from 'react-router-dom';
import { useForm } from '@mantine/form';
import { useEffect, useState } from 'react';
import { cx } from '@/utils';
import { Search as UiSearch } from '@/ui';
import styles from './Search.module.scss';
import { useNFTSearch } from '@/features/Nft/hooks';

function Search() {
  const { getInputProps, onSubmit, reset, setFieldValue } = useForm({ initialValues: { query: '' } });
  const [searchParams] = useSearchParams();
  const location = useLocation();
  const navigate = useNavigate();
  const [prevSavedPath, setPrevSavedPath] = useState('');
  const { searchQuery, resetSearchQuery } = useNFTSearch();

  const handleSubmit = onSubmit(({ query }) => {
    if (query) {
      setPrevSavedPath(location.pathname);
      searchParams.set('query', query);

      navigate({ pathname: '/search', search: searchParams.toString() });
    }
    if (!query) {
      navigate(prevSavedPath);
      setPrevSavedPath('');
    }
  });

  const handleResetButtonClick = () => {
    reset();
    resetSearchQuery();
  };

  useEffect(() => {
    setFieldValue('query', searchQuery);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchQuery]);

  return (
    <form onSubmit={handleSubmit}>
      <UiSearch {...getInputProps('query')} />
    </form>
  );
}

export { Search };
