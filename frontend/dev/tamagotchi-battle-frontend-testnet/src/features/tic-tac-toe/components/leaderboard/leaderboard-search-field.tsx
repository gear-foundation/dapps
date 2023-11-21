import { InputHTMLAttributes, useEffect } from 'react'
import { useForm } from '@mantine/form'
import { Input } from '@/components/ui/input'
import styles from '@/features/tic-tac-toe/components/leaderboard/leaderboard.module.scss'

type LeaderboardSearchFieldProps = {
  onSearch: (value: string | number) => void
  debounce?: number
} & Omit<InputHTMLAttributes<HTMLInputElement>, 'onChange'>

export function LeaderboardSearchField({
  onSearch,
  debounce = 500,
  ...props
}: LeaderboardSearchFieldProps) {
  const {
    getInputProps,
    values: { search },
  } = useForm({ initialValues: { search: '' } })

  useEffect(() => {
    const timeout = setTimeout(() => {
      onSearch(search)
    }, debounce)

    return () => clearTimeout(timeout)
  }, [search])

  return (
    <Input {...getInputProps('search')} {...props} className={styles.input} />
  )
}
