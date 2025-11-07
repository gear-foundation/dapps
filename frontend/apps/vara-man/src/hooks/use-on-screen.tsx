import { RefObject, useEffect, useMemo, useState } from 'react';

export default function useOnScreen(ref: RefObject<HTMLElement | null>) {
  const [isIntersecting, setIntersecting] = useState(false);

  const observer = useMemo(() => new IntersectionObserver(([entry]) => setIntersecting(entry.isIntersecting)), []);

  useEffect(() => {
    const element = ref.current;

    if (!element) {
      return;
    }

    observer.observe(element);

    return () => {
      observer.unobserve(element);
    };
  }, [observer, ref]);

  return isIntersecting;
}
