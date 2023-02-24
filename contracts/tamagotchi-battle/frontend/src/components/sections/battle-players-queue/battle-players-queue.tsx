import { TamagotchiQueueCard } from 'components/cards/tamagotchi-queue-card';
import 'keen-slider/keen-slider.min.css';
import { KeenSliderOptions, useKeenSlider } from 'keen-slider/react';
import { useEffect, useRef, useState } from 'react';
import { Icon } from 'components/ui/icon';
import { useBattle } from 'app/context';
import { useRefDimensions } from 'app/hooks/use-ref-dimensions';

const SPACING = 8;
const CARD_WIDTH = 160;

export const BattlePlayersQueue = () => {
  const { players } = useBattle();
  const [isSlider, setIsSlider] = useState(false);
  const ref = useRef<HTMLElement>(null);
  const [w] = useRefDimensions(ref);

  useEffect(() => {
    setIsSlider(
      players.length >
        Math.floor(w / ((players.length * CARD_WIDTH + (players.length - 1) * SPACING) / players.length)),
    );
  }, [players, w]);

  return (
    <section
      ref={ref}
      className="flex justify-center items-end mt-auto px-5 overflow-hidden min-h-[164px] xxl:min-h-[208px]">
      {isSlider ? (
        <QueueSlider />
      ) : (
        <ul className="flex gap-4 justify-center">
          {players.length > 0 &&
            players.map((item, i) => (
              <li key={i} className="w-40" style={{ width: CARD_WIDTH }}>
                <TamagotchiQueueCard tamagotchi={item} />
              </li>
            ))}
        </ul>
      )}
    </section>
  );
};

const options: KeenSliderOptions = {
  loop: true,
  mode: 'snap',
  slides: {
    perView: 'auto',
    spacing: SPACING,
  },
  created() {},
};

const QueueSlider = () => {
  const { players } = useBattle();
  const [sliderRef, instanceRef] = useKeenSlider(options);

  useEffect(() => {
    instanceRef.current?.update(options);
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
    <div className="grid gap-4 xxl:gap-6 grow">
      <div className="flex gap-4 xxl:gap-6">
        <button onClick={handlePrev} className="btn btn--primary-outline text-primary p-2 xxl:p-2.5 rounded-lg">
          <Icon name="prev" className="w-3.5 xxl:w-4.5 aspect-square" />
        </button>
        <button onClick={handleNext} className="btn btn--primary-outline text-primary p-2 xxl:p-2.5 rounded-lg">
          <Icon name="prev" className="w-3.5 xxl:w-4.5 aspect-square rotate-180" />
        </button>
      </div>
      <ul ref={sliderRef} className="keen-slider !overflow-visible">
        {players.length > 0 &&
          players.map((item, i) => (
            <li key={i} className="keen-slider__slide" style={{ width: CARD_WIDTH, minWidth: CARD_WIDTH }}>
              <div className="w-40">
                <TamagotchiQueueCard tamagotchi={item} />
              </div>
            </li>
          ))}
      </ul>
    </div>
  );
};
