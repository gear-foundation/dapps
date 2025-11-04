import { useEffect, useRef } from 'react';

import { useLessons } from '@/app/context';
import { LessonState } from '@/app/types/lessons';
import { useLessonAssets } from '@/app/utils/get-lesson-assets';

const STORAGE_KEY = 'tmgState';

export function useLessonsInit() {
  const { setLesson, lesson } = useLessons();
  const isParsed = useRef(false);
  const assets = useLessonAssets();

  useEffect(() => {
    if (lesson && assets.length) {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(lesson));
      return;
    }

    if (!isParsed.current) {
      const ls = localStorage.getItem(STORAGE_KEY);

      if (ls) {
        try {
          const parsedLesson = JSON.parse(ls) as LessonState;
          setLesson(parsedLesson);
          isParsed.current = true;
        } catch (error) {
          console.error('Failed to parse lesson from storage', error);
          localStorage.removeItem(STORAGE_KEY);
        }
      }
    } else {
      localStorage.removeItem(STORAGE_KEY);
    }
  }, [assets, lesson, setLesson]);
}
