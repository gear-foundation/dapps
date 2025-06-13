import { useEffect, useRef, useState } from 'react';

import { BannerLock } from '@/assets/images';
import { default as cardBack } from '@/assets/images/icons/cards/card-back-gold.svg';

import styles from './cards-loader.module.scss';

const TOTAL_CARDS = 24;

interface Card {
  rotate: number;
}

type CardsLoaderProps = {
  children: React.ReactNode;
};

const CardsLoader = ({ children }: CardsLoaderProps) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationFrameRef = useRef<number>();
  const [cards] = useState<Card[]>(() =>
    Array(TOTAL_CARDS)
      .fill(null)
      .map(() => ({ rotate: 0 })),
  );
  const [cardImage, setCardImage] = useState<HTMLImageElement | null>(null);

  const loadImage = (url: string): Promise<HTMLImageElement> => {
    return new Promise((resolve) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.src = url;
    });
  };

  const render = () => {
    const canvas = canvasRef.current;
    if (!canvas || !cardImage) return;

    const context = canvas.getContext('2d');
    if (!context) return;

    // Higher image smoothing quality
    context.imageSmoothingQuality = 'high';

    // Clear canvas
    context.clearRect(0, 0, canvas.width, canvas.height);

    // Loop through cards
    cards.forEach((card) => {
      // Set canvas rotation
      context.translate(canvas.width / 2, canvas.height / 2);
      context.rotate((Math.round(card.rotate) * Math.PI) / 180);
      context.translate(-canvas.width / 2, -canvas.height / 2);

      // Calculate card size
      const cardHeight = canvas.height * 0.45;
      const cardWidth = cardHeight * (cardImage.width / cardImage.height);

      // Calculate params
      const x = canvas.width / 2 - cardWidth / 2;
      const y = canvas.height / 2 - cardHeight;
      const width = cardWidth;
      const height = cardHeight;

      // Draw card image
      context.drawImage(cardImage, 0, 0, cardImage.width, cardImage.height, x, y, width, height);

      // Reset transform
      context.setTransform(1, 0, 0, 1, 0, 0);
    });

    animationFrameRef.current = requestAnimationFrame(render);
  };

  const resize = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const parent = canvas.parentElement;
    if (!parent) return;

    const { height, width } = parent.getBoundingClientRect();
    canvas.height = height;
    canvas.width = width;
  };

  const animate = () => {
    let startTime: number | null = null;
    const duration = 900;
    const pauseDuration = 100;

    const step = (timestamp: number) => {
      if (!startTime) startTime = timestamp;
      const progress = timestamp - startTime;
      const totalDuration = duration * 2 + pauseDuration * 2;

      // Fan out
      if (progress < duration) {
        cards.forEach((card, index) => {
          const targetRotation = index * (360 / (TOTAL_CARDS - 1));
          card.rotate = (targetRotation * progress) / duration;
        });
      }
      // Pause after fan out
      else if (progress < duration + pauseDuration) {
        cards.forEach((card, index) => {
          const targetRotation = index * (360 / (TOTAL_CARDS - 1));
          card.rotate = targetRotation;
        });
      }
      // Fan in
      else if (progress < duration * 2 + pauseDuration) {
        const fanInProgress = progress - (duration + pauseDuration);
        cards.forEach((card, index) => {
          const startRotation = index * (360 / (TOTAL_CARDS - 1));
          if (index === 0) {
            card.rotate = 0;
          } else {
            const clockwiseRotation = 360 - startRotation;
            card.rotate = startRotation + (clockwiseRotation * fanInProgress) / duration;
          }
        });
      }
      // Pause after fan in
      else if (progress < totalDuration) {
        cards.forEach((card) => {
          card.rotate = 0;
        });
      } else {
        startTime = null;
      }

      animationFrameRef.current = requestAnimationFrame(step);
    };

    animationFrameRef.current = requestAnimationFrame(step);
  };

  useEffect(() => {
    void loadImage(cardBack).then((img) => {
      setCardImage(img);
    });
  }, []);

  useEffect(() => {
    if (!cardImage) return;

    const init = () => {
      resize();
      animate();
      render();
    };

    void init();

    window.addEventListener('resize', resize);

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
      window.removeEventListener('resize', resize);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [cardImage]);

  return (
    <section className={styles.section}>
      <div className={styles.cardFan}>
        {cardImage && <div className={styles.glow} />}
        <canvas ref={canvasRef} className={styles.canvas} />
        <BannerLock className={styles.bannerLock} />
      </div>
      {children}
    </section>
  );
};

export { CardsLoader };
