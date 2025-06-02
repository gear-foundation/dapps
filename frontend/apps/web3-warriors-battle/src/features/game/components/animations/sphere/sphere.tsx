import clsx from 'clsx';
import React, { useEffect, useRef } from 'react';

import { Move } from '@/app/utils';

import styles from './sphere.module.scss';

const PARTICLE_RADIUS = 20;

interface Particle {
  speed: { x: number; y: number };
  location: { x: number; y: number };
  radius: number;
  life: number;
  remaining_life: number;
  r: number;
  g: number;
  b: number;
  opacity?: number;
}

class ParticleFlame implements Particle {
  speed;
  location;
  radius;
  life;
  remaining_life;
  r;
  g;
  b;

  constructor(W: number, H: number, flamewidth: number, type: Move) {
    this.speed = { x: -2.5 + Math.random() * 5, y: -15 + Math.random() * 10 };
    const locmin = W / 2 - flamewidth / 2;
    const locmax = W / 2 + flamewidth / 2;
    this.location = { x: Math.random() * (locmax - locmin) + locmin, y: H };

    this.radius = Math.random() * PARTICLE_RADIUS + PARTICLE_RADIUS;
    this.life = 10 + Math.random() * 10;
    this.remaining_life = this.life;

    if (type === 'Attack') {
      this.r = 255;
      this.g = Math.round(Math.random() * 90 + 100);
      this.b = Math.round(Math.random() * 20 + 10);
    } else {
      this.r = Math.round(Math.random() * 30 + 215);
      this.g = Math.round(Math.random() * 20 + 215);
      this.b = 255;
    }
  }
}

type FireAnimationProps = {
  type?: Move | null;
  className?: string;
};

export const SphereAnimation: React.FC<FireAnimationProps> = ({ type, className }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const W = 360;
    const H = 360;
    canvas.width = W;
    canvas.height = H;

    if (!type || type === 'Reflect') {
      ctx.clearRect(0, 0, W, H);
      return;
    }

    const particles: Particle[] = [];
    const particle_count = 100;
    const flamewidth = 360;

    for (let i = 0; i < particle_count; i++) {
      particles.push(new ParticleFlame(W, H, flamewidth, type));
    }

    function drawFlames() {
      if (!ctx || !type) return;
      ctx.globalCompositeOperation = 'source-over';
      ctx.clearRect(0, 0, W, H);
      ctx.globalCompositeOperation = 'lighter';

      for (let i = 0; i < particles.length; i++) {
        const p = particles[i];
        ctx.beginPath();
        p.opacity = Math.round((p.remaining_life / p.life) * 100) / 100;
        const gradient = ctx.createRadialGradient(p.location.x, p.location.y, 0, p.location.x, p.location.y, p.radius);
        gradient.addColorStop(0, `rgba(${p.r}, ${p.g}, ${p.b}, ${p.opacity})`);
        gradient.addColorStop(0.5, `rgba(${p.r}, ${p.g}, ${p.b}, ${p.opacity})`);
        gradient.addColorStop(1, `rgba(${p.r}, ${p.g}, ${p.b}, 0)`);
        ctx.fillStyle = gradient;
        ctx.arc(p.location.x, p.location.y, p.radius, Math.PI * 2, 0, false);
        ctx.fill();

        p.remaining_life--;
        p.radius--;
        p.location.x += p.speed.x;
        p.location.y += p.speed.y;

        if (p.remaining_life < 0 || p.radius < 0) {
          particles[i] = new ParticleFlame(W, H, flamewidth, type);
        }
      }
    }

    const interval = setInterval(drawFlames, 33);

    return () => clearInterval(interval);
  }, [type]);

  return (
    <canvas
      ref={canvasRef}
      className={clsx(className, {
        [styles.attack]: type === 'Attack',
        [styles.ultimate]: type === 'Ultimate',
        [styles.reflect]: type === 'Reflect',
      })}
    />
  );
};
