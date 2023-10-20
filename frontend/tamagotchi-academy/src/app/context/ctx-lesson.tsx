import { createContext, PropsWithChildren, useState } from 'react'
import { LessonState } from '@/app/types/lessons'
import { useProgramMetadata } from '../hooks/use-metadata'
import meta5 from '@/assets/meta/meta5.txt'
// import { ProgramMetadata } from '@gear-js/api'

const key = 'tmgState'

const useProgram = () => {
  const [lesson, setLesson] = useState<LessonState>()
  // const [lessonMeta, setLessonMeta] = useState<ProgramMetadata>()
  const [isAdmin, setIsAdmin] = useState<boolean>(false)
  const [isReady, setIsReady] = useState<boolean>(false)
  const resetLesson = () => {
    setLesson(undefined)
    setIsAdmin(false)
    setIsReady(false)
    localStorage.removeItem(key)
  }

  const lessonMeta = useProgramMetadata(meta5)

  return {
    lesson,
    setLesson,
    lessonMeta,
    // setLessonMeta,
    isAdmin,
    setIsAdmin,
    isReady,
    setIsReady,
    resetLesson,
  }
}

export const LessonsCtx = createContext({} as ReturnType<typeof useProgram>)

export function LessonsProvider({ children }: PropsWithChildren) {
  const { Provider } = LessonsCtx
  return <Provider value={useProgram()}>{children}</Provider>
}
