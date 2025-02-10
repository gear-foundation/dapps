import { useProgramMetadata } from '@/app/hooks/use-metadata';
import meta1 from '@/assets/meta/meta1.txt';
import meta2 from '@/assets/meta/meta2.txt';

export function useLessonAssets() {
  return [useProgramMetadata(meta1), useProgramMetadata(meta2)];
}
