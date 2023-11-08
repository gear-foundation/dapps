import { MutableRefObject, useEffect, useState } from 'react';

function debounce(fn: any, ms: number) {
  let timer: number | null | ReturnType<typeof setTimeout>;
  return (_: any) => {
    clearTimeout(timer as number);
    timer = setTimeout((_) => {
      timer = null;
      // @ts-ignore
      fn.apply(this, arguments);
    }, ms);
  };
}
const useRefDimensions = (ref: MutableRefObject<HTMLElement | null>) => {
  const [dimensions, setDimensions] = useState([0, 0]);

  useEffect(() => {
    const handleResize = debounce(() => {
      if (ref.current) {
        const { width, height } = ref.current.getBoundingClientRect();
        setDimensions([width, height]);
      }
    }, 150);

    handleResize(() => {});

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [ref]);

  return dimensions;
};

export { useRefDimensions };
