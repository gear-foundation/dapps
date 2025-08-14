import clsx from 'clsx';
import { useState, useRef, useEffect, useCallback } from 'react';

import styles from './slider.module.scss';

export type Option = {
  value: number;
  label: string;
};

type Props = {
  label?: string;
  options: Option[];
  defaultValue?: number;
  onChange?: (value: number) => void;
  className?: string;
};

const Slider = ({ label, options, defaultValue, onChange, className }: Props) => {
  const [selectedValue, setSelectedValue] = useState(defaultValue || options[0].value);
  const [isDragging, setIsDragging] = useState(false);
  const trackRef = useRef<HTMLDivElement>(null);
  const thumbRef = useRef<HTMLDivElement>(null);

  const handleChange = useCallback(
    (value: number) => {
      setSelectedValue(value);
      if (onChange) onChange(value);
    },
    [onChange],
  );

  const getSliderPosition = () => {
    const index = options.findIndex((option) => option.value === selectedValue);
    if (index === -1) return 0;

    const totalSteps = options.length - 1;
    const step = 100 / totalSteps;
    let position = index * step;

    if (index !== 0 && index !== 1 && index !== totalSteps) {
      position -= index;
    }

    return position;
  };

  const findClosestValue = useCallback(
    (position: number) => {
      if (!trackRef.current) return options[0].value;

      const trackWidth = trackRef.current.clientWidth;
      const relativePosition = Math.max(0, Math.min(position, trackWidth)) / trackWidth;

      const index = Math.round(relativePosition * (options.length - 1));
      return options[index].value;
    },
    [options],
  );

  const handleTrackClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!trackRef.current) return;

    const rect = trackRef.current.getBoundingClientRect();
    const position = e.clientX - rect.left;

    const newValue = findClosestValue(position);
    handleChange(newValue);
  };

  const handleThumbMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsDragging(true);
  };

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging || !trackRef.current) return;

      const rect = trackRef.current.getBoundingClientRect();
      const position = e.clientX - rect.left;

      const newValue = findClosestValue(position);
      handleChange(newValue);
    };

    const handleMouseUp = () => {
      setIsDragging(false);
    };

    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, options, handleChange, findClosestValue]);

  useEffect(() => {
    const thumb = thumbRef.current;
    if (!thumb) return;

    const handleTouchStart = () => {
      setIsDragging(true);
    };

    const handleTouchMove = (e: TouchEvent) => {
      if (!isDragging || !trackRef.current) return;

      const rect = trackRef.current.getBoundingClientRect();
      const position = e.touches[0].clientX - rect.left;

      const newValue = findClosestValue(position);
      handleChange(newValue);
    };

    const handleTouchEnd = () => {
      setIsDragging(false);
    };

    thumb.addEventListener('touchstart', handleTouchStart);
    document.addEventListener('touchmove', handleTouchMove);
    document.addEventListener('touchend', handleTouchEnd);

    return () => {
      thumb?.removeEventListener('touchstart', handleTouchStart);
      document.removeEventListener('touchmove', handleTouchMove);
      document.removeEventListener('touchend', handleTouchEnd);
    };
  }, [isDragging, options, handleChange, findClosestValue]);

  return (
    <div className={clsx(styles.container, className)}>
      <div className={styles.label}>{label}</div>

      <div className={clsx(styles.sliderContainer, isDragging && styles.dragging)}>
        <div
          ref={trackRef}
          className={styles.track}
          onClick={handleTrackClick}
          role="slider"
          tabIndex={0}
          aria-valuemin={options[0].value}
          aria-valuemax={options[options.length - 1].value}
          aria-valuenow={selectedValue}
          onKeyDown={(e) => {
            if (e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
              const currentIndex = options.findIndex((opt) => opt.value === selectedValue);
              const newIndex =
                e.key === 'ArrowLeft' ? Math.max(0, currentIndex - 1) : Math.min(options.length - 1, currentIndex + 1);
              handleChange(options[newIndex].value);
            }
          }}>
          <div className={styles.progress} style={{ width: `${getSliderPosition()}%` }} />
          <div
            ref={thumbRef}
            className={clsx(styles.thumb, isDragging && styles.dragging)}
            style={{ left: `${getSliderPosition()}%` }}
            onMouseDown={handleThumbMouseDown}
            role="presentation"
          />
        </div>

        <div className={styles.values}>
          {options.map((option) => (
            <button
              key={option.value}
              type="button"
              className={clsx(styles.valueItem, selectedValue === option.value && styles.selected)}
              onClick={() => handleChange(option.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  handleChange(option.value);
                }
              }}>
              <div className={styles.valueLabel}>{option.label}</div>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
};

export { Slider };
