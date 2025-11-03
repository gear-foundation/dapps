import { RefObject, useEffect, useState } from 'react';

type DebouncedCallback = () => void;

function debounce(fn: DebouncedCallback, ms: number) {
  let timer: ReturnType<typeof setTimeout> | null = null;

  return () => {
    if (timer) {
      clearTimeout(timer);
    }

    timer = setTimeout(() => {
      timer = null;
      fn();
    }, ms);
  };
}

const useRefDimensions = (ref: RefObject<HTMLElement | null>) => {
  const [dimensions, setDimensions] = useState<[number, number]>([0, 0]);

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
