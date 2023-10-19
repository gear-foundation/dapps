import { useForm } from '@mantine/form'
import { useNavigate, useSearchParams } from 'react-router-dom'
import { useEffect } from 'react'
import { Button } from 'components'
import { ReactComponent as SearchSVG } from '../../assets/search.svg'
import { ReactComponent as ResetSVG } from '../../assets/reset.svg'
import { useNFTSearch } from '../../hooks'
import styles from './Search.module.scss'

export function Search() {
  const { searchQuery, resetSearchQuery } = useNFTSearch()
  const { getInputProps, onSubmit, reset, setFieldValue } = useForm({
    initialValues: { query: '' },
  })

  const [searchParams] = useSearchParams()
  const navigate = useNavigate()

  const handleSubmit = onSubmit(({ query }) => {
    searchParams.set('query', query)

    navigate({ pathname: '/list', search: searchParams.toString() })
  })

  const handleResetButtonClick = () => {
    reset()
    resetSearchQuery()
  }

  useEffect(() => {
    setFieldValue('query', searchQuery)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchQuery])

  return (
    <form className={styles.inputWrapper} onSubmit={handleSubmit}>
      <SearchSVG />
      {/* eslint-disable-next-line react/jsx-props-no-spreading */}
      <input
        type="text"
        placeholder="Search NFTs and accounts"
        id="search"
        {...getInputProps('query')}
      />

      {searchQuery && (
        <Button
          variant="text"
          onClick={handleResetButtonClick}
          className={styles.reset}
        >
          <ResetSVG />
        </Button>
      )}
    </form>
  )
}
