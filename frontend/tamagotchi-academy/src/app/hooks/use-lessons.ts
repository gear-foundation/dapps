import { useEffect, useRef } from 'react'
import { useLessonAssets } from '@/app/utils/get-lesson-assets'
import { useLessons } from '@/app/context'

const key = 'tmgState'

export function useLessonsInit() {
  const { setLesson, setLessonMeta, lesson } = useLessons()
  const isParsed = useRef(false)
  const assets = useLessonAssets()

  useEffect(() => {
    if (lesson && assets.length) {
      localStorage.setItem(key, JSON.stringify(lesson))
      setLessonMeta(assets[+lesson.step || 0])
    } else {
      if (!isParsed.current) {
        const ls = localStorage.getItem(key)
        if (ls) {
          setLesson(JSON.parse(ls))
          isParsed.current = true
        }
      } else localStorage.removeItem(key)
    }
  }, [assets, lesson])
}
