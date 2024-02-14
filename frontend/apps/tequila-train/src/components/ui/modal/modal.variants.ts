import { Variants } from 'framer-motion';

export const variantsOverlay: Variants = {
  closed: {
    opacity: 0,
    transition: {
      // delay: 0.15,
      duration: 0.3,
      ease: 'easeIn',
    },
  },
  open: {
    opacity: 1,
    transition: {
      duration: 0.2,
      ease: 'easeOut',
    },
  },
};
export const variantsPanel: Variants = {
  closed: {
    y: 'var(--y-closed, 0)',
    opacity: 'var(--opacity-closed)',
    scale: 'var(--scale-closed, 1)',
    transition: {
      duration: 0.3,
      ease: 'easeIn',
    },
  },
  open: {
    y: 'var(--y-open, 0)',
    opacity: 'var(--opacity-open)',
    scale: 'var(--scale-open, 1)',
    transition: {
      // delay: 0.15,
      duration: 0.2,
    },
  },
};
