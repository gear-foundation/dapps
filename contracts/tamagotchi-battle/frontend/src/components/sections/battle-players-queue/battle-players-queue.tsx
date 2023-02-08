import { TamagotchiQueueCard } from '../../cards/tamagotchi-queue-card';
import 'keen-slider/keen-slider.min.css';
import { useKeenSlider } from 'keen-slider/react';
import { useEffect, useRef, useState } from 'react';
import { Icon } from '../../ui/icon';
import { useBattle } from 'app/context';
import { BattlePlayerType } from '../../../app/types/battles';
import { useRefDimensions } from '../../../app/hooks/use-ref-dimensions';

export const BattlePlayersQueue = () => {
  const { battleState: battle } = useBattle();
  const [queue, setQueue] = useState<BattlePlayerType[]>([]);
  const ref = useRef<HTMLElement>(null);
  const [w] = useRefDimensions(ref);

  const [currentSlide, setCurrentSlide] = useState(0);
  const [loaded, setLoaded] = useState(false);
  const [sliderRef, instanceRef] = useKeenSlider(
    {
      loop: true,
      mode: 'snap',
      slides: {
        perView: 'auto',
        spacing: 15,
      },
      slideChanged(slider) {
        setCurrentSlide(slider.track.details.rel);
      },
      created() {
        setLoaded(true);
      },
    },
    [
      // add plugins here
    ],
  );

  useEffect(() => {
    if (battle?.players) {
      setQueue(Object.values(battle.players));
    } else {
      setQueue([]);
    }
  }, [battle]);

  const handlePrev = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    instanceRef.current?.prev();
  };

  const handleNext = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    instanceRef.current?.next();
  };

  return (
    <section className="mt-auto px-5 flex gap-4 xl:gap-6" ref={ref}>
      {queue.length > Math.floor(w / 160) && loaded && instanceRef.current && (
        <div className="flex gap-4 xl:gap-6">
          <button onClick={handlePrev} className="btn btn--primary-outline text-primary p-2 xl:p-2.5 rounded-lg">
            <Icon name="prev" className="w-3.5 xl:w-4.5 aspect-square" />
          </button>
          <button onClick={handleNext} className="btn btn--primary-outline text-primary p-2 xl:p-2.5 rounded-lg">
            <Icon name="prev" className="w-3.5 xl:w-4.5 aspect-square rotate-180" />
          </button>
        </div>
      )}
      <ul ref={sliderRef} className="keen-slider">
        {queue.length > 0 &&
          queue.map((item, i) => (
            <li key={i} className="keen-slider__slide" style={{ width: 160 }}>
              <div className="w-40">
                <TamagotchiQueueCard className="" tamagotchi={item} />
              </div>
            </li>
          ))}
      </ul>
    </section>
  );
};
