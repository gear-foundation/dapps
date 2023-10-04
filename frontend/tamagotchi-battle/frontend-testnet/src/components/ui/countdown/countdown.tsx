import { useState, useEffect } from 'react'
import { getHours, getMinutes, getSeconds } from '@/app/utils'

export const getTime = (value: number | undefined) => {
  if (!value) return '00:00:00'

  return `${getHours(value)}:${getMinutes(value)}:${getSeconds(value)}`
}

export function useCountdown({ time }: { time: number }) {
  const [countdown, setCountdown] = useState(time)

  useEffect(() => {
    if (!countdown) return

    const timer = setInterval(() => {
      setCountdown((prevCountdown) => {
        const updatedCountdown = prevCountdown - 1000
        return updatedCountdown <= 0 ? 0 : updatedCountdown
      })
    }, 1000)

    return () => {
      clearInterval(timer)
    }
  }, [countdown])

  return {
    countdown,
    trigger: (value: number | undefined = time) => setCountdown(value),
  }
}
