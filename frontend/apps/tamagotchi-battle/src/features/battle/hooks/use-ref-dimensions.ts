import { useEffect, useState, type RefObject } from 'react';

function debounce<T extends (...args: unknown[]) => void>(fn: T, ms: number) {
  let timer: ReturnType<typeof setTimeout> | null = null;

  return (...args: Parameters<T>) => {
    if (timer) {
      clearTimeout(timer);
    }

    timer = setTimeout(() => {
      timer = null;
      fn(...args);
    }, ms);
  };
}
const useRefDimensions = (ref: RefObject<HTMLElement | null>) => {
  const [dimensions, setDimensions] = useState([0, 0]);

  useEffect(() => {
    const handleResize = debounce(() => {
      if (ref.current) {
        const { width, height } = ref.current.getBoundingClientRect();
        setDimensions([width, height]);
      }
    }, 150);

    handleResize();

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [ref]);

  return dimensions;
};

export { useRefDimensions };
