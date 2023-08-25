import { useForm } from '@mantine/form';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { Button } from '@gear-js/ui';
import { useEffect } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { ReactComponent as SearchSVG } from '../../assets/search.svg';
import { ReactComponent as ResetSVG } from '../../assets/reset.svg';
import { useGetAllNFTs, useNFTSearch } from '../../hooks';
import styles from './Search.module.scss';
import { usePendingUI } from '../../../../hooks';

function Search() {
  const { isAccountReady } = useAccount();
  const { searchQuery, resetSearchQuery } = useNFTSearch();
  const { getInputProps, onSubmit, reset, setFieldValue } = useForm({ initialValues: { query: '' } });

  const [searchParams] = useSearchParams();
  const navigate = useNavigate();

  const { getAllNFTs, isStateRead } = useGetAllNFTs();
  const { setIsPending } = usePendingUI();

  const handleSubmit = onSubmit(({ query }) => {
    searchParams.set('query', query);

    if (!isStateRead) {
      setIsPending(true);
      getAllNFTs(() => {
        setIsPending(false);
      });
    }

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

  return isAccountReady ? (
    <form className={styles.inputWrapper} onSubmit={handleSubmit}>
      <SearchSVG />
      {/* eslint-disable-next-line react/jsx-props-no-spreading */}
      <input type="text" placeholder="Search NFTs and accounts" id="search" {...getInputProps('query')} />

      {searchQuery && <Button icon={ResetSVG} color="transparent" onClick={handleResetButtonClick} />}
    </form>
  ) : null;
}

export { Search };
