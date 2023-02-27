import { TamagotchiQueueCard } from 'components/cards/tamagotchi-queue-card';
import 'keen-slider/keen-slider.min.css';
import { KeenSliderOptions, useKeenSlider } from 'keen-slider/react';
import { useEffect, useRef, useState } from 'react';
import { Icon } from 'components/ui/icon';
import { useBattle } from 'app/context';
import { useRefDimensions } from 'app/hooks/use-ref-dimensions';
import { useIsLarge } from '../../../app/hooks/use-media';
import { PLAYER_CARD } from '../../../app/consts';
import { AnimatePresence, motion } from 'framer-motion';

const container = {
  hidden: { opacity: 0 },
  show: {
    opacity: 1,
    transition: {
      staggerChildren: 0.5,
    },
  },
};

const itemV = {
  hidden: { opacity: 0, y: 20 },
  show: { opacity: 1, y: 0 },
};

export const BattlePlayersQueue = () => {
  const { players } = useBattle();
  const [isSlider, setIsSlider] = useState(false);
  const ref = useRef<HTMLElement>(null);
  const init = useRef(false);
  const [w] = useRefDimensions(ref);
  const isFull = useIsLarge();

  const width = isFull ? PLAYER_CARD.width.desktop : PLAYER_CARD.width.mobile;
  const space = isFull ? PLAYER_CARD.spacing.desktop : PLAYER_CARD.spacing.mobile;

  useEffect(() => {
    setIsSlider(
      players.length > Math.floor(w / ((players.length * width + (players.length - 1) * space) / players.length)),
    );
  }, [players, space, w, width]);

  useEffect(() => {
    if (!init.current && players && w) {
      init.current = true;
    }
  }, [players, w]);

  return (
    <section
      ref={ref}
      className="flex justify-center items-end mt-auto px-5 overflow-hidden min-h-[144px] xxl:min-h-[208px]">
      <AnimatePresence>
        {init.current &&
          (isSlider ? (
            <QueueSlider />
          ) : (
            <motion.ol
              key="queue-container"
              variants={container}
              initial="hidden"
              animate="show"
              className="flex gap-3 xxl:gap-2 justify-center">
              {players.length > 0 &&
                players.map((item, i) => (
                  <motion.li key={i} className="w-35 xxl:w-40" style={{ width: width }} variants={itemV}>
                    <TamagotchiQueueCard tamagotchi={item} />
                  </motion.li>
                ))}
            </motion.ol>
          ))}
      </AnimatePresence>
    </section>
  );
};

const options: KeenSliderOptions = {
  loop: true,
  mode: 'snap',
  slides: {
    perView: 'auto',
    spacing: PLAYER_CARD.spacing.desktop,
  },
  created() {},
};

const QueueSlider = () => {
  const { players } = useBattle();
  const [sliderRef, instanceRef] = useKeenSlider(options);
  const isFull = useIsLarge();

  const width = isFull ? PLAYER_CARD.width.desktop : PLAYER_CARD.width.mobile;
  const space = isFull ? PLAYER_CARD.spacing.desktop : PLAYER_CARD.spacing.mobile;

  useEffect(() => {
    instanceRef.current?.update({ ...options, slides: { perView: 'auto', spacing: space } });
  }, [instanceRef]);

  const handlePrev = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    instanceRef.current?.prev();
  };

  const handleNext = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    instanceRef.current?.next();
  };

  return (
    <div className="grow w-full space-y-3 xxl:space-y-6">
      <div className="flex gap-4 xxl:gap-6">
        <button onClick={handlePrev} className="btn btn--primary-outline text-primary p-2 xxl:p-2.5 rounded-lg">
          <Icon name="prev" className="w-3.5 xxl:w-4.5 aspect-square" />
        </button>
        <button onClick={handleNext} className="btn btn--primary-outline text-primary p-2 xxl:p-2.5 rounded-lg">
          <Icon name="prev" className="w-3.5 xxl:w-4.5 aspect-square rotate-180" />
        </button>
      </div>
      <motion.ol
        variants={container}
        initial="hidden"
        animate="show"
        key="queue-slider-list"
        ref={sliderRef}
        className="keen-slider !overflow-visible">
        {players.length > 0 &&
          players.map((item, i) => (
            <motion.li
              variants={itemV}
              key={i}
              className="keen-slider__slide"
              style={{ width: width, minWidth: width }}>
              <div className="w-35 xxl:w-40">
                <TamagotchiQueueCard tamagotchi={item} />
              </div>
            </motion.li>
          ))}
      </motion.ol>
    </div>
  );
};
