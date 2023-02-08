import { TamagotchiQueueCard } from '../../cards/tamagotchi-queue-card';
import 'keen-slider/keen-slider.min.css';
import { useKeenSlider } from 'keen-slider/react';
import { useState } from 'react';
import { Icon } from '../../ui/icon';
import { useBattle } from 'app/context';

export const BattlePlayersQueue = () => {
  const { battleState: battle } = useBattle();

  const [currentSlide, setCurrentSlide] = useState(0);
  const [loaded, setLoaded] = useState(false);
  const [sliderRef, instanceRef] = useKeenSlider(
    {
      loop: true,
      mode: 'free-snap',
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

  const handlePrev = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    instanceRef.current?.prev();
  };

  const handleNext = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    instanceRef.current?.next();
  };

  return (
    <section className="mt-4 xl:mt-8 px-5">
      {loaded && instanceRef.current && (
        <div className="flex gap-4 xl:gap-6">
          <button onClick={handlePrev} className="btn btn--primary-outline text-primary p-2 xl:p-2.5 rounded-lg">
            <Icon name="prev" className="w-3.5 xl:w-4.5 aspect-square" />
          </button>
          <button onClick={handleNext} className="btn btn--primary-outline text-primary p-2 xl:p-2.5 rounded-lg">
            <Icon name="prev" className="w-3.5 xl:w-4.5 aspect-square rotate-180" />
          </button>
        </div>
      )}

      <ul ref={sliderRef} className="keen-slider mt-4 xl:mt-6">
        <li className="keen-slider__slide">
          <div className="w-40">
            <TamagotchiQueueCard className="" />
          </div>
        </li>
      </ul>
    </section>
  );
};
