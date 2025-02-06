import { useForm } from '@mantine/form';
import { useEffect } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';

import { Button } from '@/components';

import ResetSVG from '../../assets/reset.svg?react';
import SearchSVG from '../../assets/search.svg?react';
import { useNFTSearch } from '../../hooks';

import styles from './Search.module.scss';

export function Search() {
  const { searchQuery, resetSearchQuery } = useNFTSearch();
  const { getInputProps, onSubmit, reset, setFieldValue } = useForm({
    initialValues: { query: '' },
  });

  const [searchParams] = useSearchParams();
  const navigate = useNavigate();

  const handleSubmit = onSubmit(({ query }) => {
    searchParams.set('query', query);

    navigate({ pathname: '/list', search: searchParams.toString() });
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
    <form className={styles.inputWrapper} onSubmit={handleSubmit}>
      <SearchSVG />
      {}
      <input type="text" placeholder="Search NFTs and accounts" id="search" {...getInputProps('query')} />

      {searchQuery && (
        <Button variant="text" onClick={handleResetButtonClick} className={styles.reset}>
          <ResetSVG />
        </Button>
      )}
    </form>
  );
}
