import { useState } from 'react'
import { AnimatePresence, motion, wrap } from 'framer-motion'
import { Icons } from '@/components/ui/icons'

import RulesImage1 from '@/assets/images/rules/rules-1.webp'
import RulesImage2 from '@/assets/images/rules/rules-2.webp'
import RulesImage3 from '@/assets/images/rules/rules-3.webp'


const images = [RulesImage1, RulesImage2, RulesImage3];

const variants = {
  enter: (direction: number) => {
    return {
      x: direction > 0 ? 2200 : -2200,
      opacity: 0,
    };
  },
  center: {
    zIndex: 1,
    x: 0,
    opacity: 1,
  },
  exit: (direction: number) => {
    return {
      zIndex: 0,
      x: direction < 0 ? 2200 : -2200,
      opacity: 0,
    };
  },
  hidden: {
    zIndex: -1,
    opacity: 0,
  },
};

const swipeConfidenceThreshold = 10000;
const swipePower = (offset: number, velocity: number) => {
  return Math.abs(offset) * velocity;
};

export default function Rules() {
  const [[page, direction], setPage] = useState([0, 0]);
  const [animationInProgress, setAnimationInProgress] = useState(false);
  const imageIndex = wrap(0, images.length, page);

  const paginate = (newDirection: number) => {
    if (!animationInProgress) {
      setAnimationInProgress(true);
      setPage([page + newDirection, newDirection]);
    }
  };

  return (
    <div className="relative grow flex items-center justify-center w-full h-full pt-8 pb-15">
      <AnimatePresence initial={false} custom={direction}>
        <motion.img
          key={page}
          src={images[imageIndex]}
          className="absolute w-full h-full object-contain"
          custom={direction}
          variants={variants}
          initial="enter"
          animate="center"
          exit="exit"
          transition={{
            x: { type: 'spring', stiffness: 300, damping: 30 },
            opacity: { duration: 0.25 },
          }}
          drag={animationInProgress ? false : 'x'}
          dragConstraints={{ left: 0, right: 0 }}
          dragElastic={1}
          onDragEnd={(e, { offset, velocity }) => {
            const swipe = swipePower(offset.x, velocity.x);

            if (swipe < -swipeConfidenceThreshold) {
              page < images.length - 1 && paginate(1);
            } else if (swipe > swipeConfidenceThreshold) {
              page > 0 && paginate(-1);
            }
          }}
          onAnimationComplete={() => {
            setAnimationInProgress(false);
          }}
        />
      </AnimatePresence>
      <div className="absolute top-0 right-0 z-1 grid grid-cols-2 gap-5">
        {page > 0 && (
          <button
            className="col-start-1 group level-mode level-mode--to-right p-2.5 hover:[--from:#16B768]"
            onClick={() => paginate(-1)}
          >
            <span className="before:absolute before:inset-0 before:z-0 before:bg-[#1e1e1e] before:rounded-[8px] group-hover:before:bg-primary before:transition-colors" />
            <Icons.sliderPrev className="relative z-1 w-5 h-5 text-primary group-hover:text-white" />
          </button>
        )}

        {page < images.length - 1 && (
          <button
            className="col-start-2 group level-mode p-2.5 hover:[--from:#16B768]"
            onClick={() => paginate(1)}
          >
            <span className="before:transition-colors before:absolute before:inset-0 before:z-0 before:bg-[#1e1e1e] before:rounded-[8px] group-hover:before:bg-primary" />
            <Icons.sliderNext className="relative z-1 w-5 h-5 text-primary group-hover:text-white transition-colors" />
          </button>
        )}
      </div>
    </div>
  );
}