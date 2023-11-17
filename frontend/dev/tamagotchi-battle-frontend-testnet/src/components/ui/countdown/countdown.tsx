import { useState, useEffect } from 'react'

type Props = {
  milliseconds: number
  text?: string
  onClick?: () => void
}

export function Countdown({ milliseconds, text, onClick }: Props) {
  const [countdown, setCountdown] = useState(milliseconds)
  const isCountdown = !!countdown

  useEffect(() => {
    if (!isCountdown) return

    const timer = setInterval(() => {
      setCountdown((prevCountdown) => {
        const updatedCountdown = prevCountdown - 1000
        return updatedCountdown <= 0 ? 0 : updatedCountdown
      })
    }, 1000)

    return () => {
      clearInterval(timer)
    }
  }, [isCountdown])

  const getTime = (value: number | undefined) => {
    if (!value) return '00:00:00'

    const hours = Math.floor(value / (1000 * 60 * 60))
      .toString()
      .padStart(2, '0')

    const minutes = Math.floor((value % (1000 * 60 * 60)) / (1000 * 60))
      .toString()
      .padStart(2, '0')

    const seconds = Math.floor((value % (1000 * 60)) / 1000)
      .toString()
      .padStart(2, '0')

    return `${hours}:${minutes}:${seconds}`
  }

  return <>{getTime(countdown)}</>
}
