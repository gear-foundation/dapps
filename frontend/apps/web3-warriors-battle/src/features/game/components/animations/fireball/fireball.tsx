import React, { useRef, useEffect } from 'react';

import { BattleHistory } from '@/features/game/types';

interface Particle {
  index: number;
  x: number;
  y: number;
  r: number;
  o: number;
  c: string;
  xv: number;
  yv: number;
  rv: number;
  ov: number;
}

interface Fireball {
  index: number;
  x: number;
  y: number;
  xv: number;
  yv: number;
  life: number;
  reflectLeft?: number;
  reflectRight?: number;
}

type FireballCanvasProps = {
  lastTurnHistory: BattleHistory;
};

export const FireballCanvas: React.FC<FireballCanvasProps> = ({ lastTurnHistory }) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const fireballs = useRef<{ [key: number]: Fireball }>({});
  const particles = useRef<{ [key: number]: Particle }>({});
  const nextFireballIndex = useRef(0);
  const nextParticleIndex = useRef(0);
  const o = useRef({ x: 0, y: 0 });
  const edge = useRef({ top: 0, right: 0, bottom: 0, left: 0 });

  const gravity = 0.025;
  const particleColor = '#f73';
  const radius = 11;

  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext('2d');

    if (!canvas || !ctx) return;

    const resize = () => {
      const width = (canvas.width = canvas.clientWidth);
      const height = (canvas.height = canvas.clientHeight);
      o.current = { x: Math.floor(width / 2), y: Math.floor(height / 2) };
      edge.current = {
        top: -o.current.y,
        right: width - o.current.x,
        bottom: height - o.current.y,
        left: -o.current.x,
      };
    };

    resize();
    window.addEventListener('resize', resize);

    const newParticle = (
      x: number,
      y: number,
      r: number,
      o: number,
      c: string,
      xv: number,
      yv: number,
      rv: number,
      ov: number,
    ) => {
      const index = ++nextParticleIndex.current;
      particles.current[index] = { index, x, y, r, o, c, xv, yv, rv, ov };
    };

    const loop = () => {
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      ctx.globalCompositeOperation = 'source-over';
      ctx.globalAlpha = 1;
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      ctx.translate(o.current.x, o.current.y);

      ctx.globalCompositeOperation = 'lighter';
      for (const i in particles.current) {
        const p = particles.current[i];
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
        ctx.globalAlpha = p.o;
        ctx.fillStyle = p.c;
        ctx.fill();
      }

      for (const i in particles.current) {
        const p = particles.current[i];
        p.x += p.xv;
        p.y += p.yv;
        p.r += p.rv;
        p.o += p.ov;
        if (p.r < 0 || p.o < 0) delete particles.current[p.index];
      }

      for (const i in fireballs.current) {
        const f = fireballs.current[i];
        const numParticles = Math.max(Math.sqrt(f.xv * f.xv + f.yv * f.yv) / 5, 1);
        const numParticlesInt = Math.ceil(numParticles);
        const numParticlesDif = numParticles / numParticlesInt;

        for (let j = 0; j < numParticlesInt; j++) {
          newParticle(
            f.x - (f.xv * j) / numParticlesInt,
            f.y - (f.yv * j) / numParticlesInt,
            radius,
            numParticlesDif,
            particleColor,
            Math.random() * 0.6 - 0.3,
            Math.random() * 0.6 - 0.3,
            -0.3,
            -0.05 * numParticlesDif,
          );
        }

        f.x += f.xv;
        f.y += f.yv;
        f.yv += gravity;

        let boundary;
        if (f.y < (boundary = edge.current.top + 7)) {
          f.y = boundary;
          f.yv *= -1;
        } else if (f.y > (boundary = edge.current.bottom - 7)) {
          f.y = boundary;
          f.yv *= -1;
        }
        if (f.x > (boundary = f.reflectLeft || edge.current.right - 7)) {
          f.x = boundary;
          f.xv *= -1.5;
          f.yv *= -1;
        } else if (f.x < (boundary = f.reflectRight || edge.current.left + 7)) {
          f.x = boundary;
          f.xv *= -1.5;
          f.yv *= -1;
        }

        if (--f.life < 0) delete fireballs.current[f.index];
      }

      requestAnimationFrame(loop);
    };

    loop();

    return () => {
      window.removeEventListener('resize', resize);
    };
  }, []);

  const launchFireball = (
    x: number,
    y: number,
    xv: number,
    yv: number,
    life: number,
    reflectLeft?: number,
    reflectRight?: number,
  ) => {
    fireballs.current[++nextFireballIndex.current] = {
      index: nextFireballIndex.current,
      x,
      y,
      xv,
      yv,
      life,
      reflectLeft,
      reflectRight,
    };
  };

  useEffect(() => {
    const playerAttack = ['Attack', 'Ultimate'].includes(lastTurnHistory.player.action || '');
    const opponentAttack = ['Attack', 'Ultimate'].includes(lastTurnHistory.opponent.action || '');

    if (playerAttack) {
      if (lastTurnHistory.opponent.isDodged) {
        launchFireball(-250, 200, 2.46, -3.03, 400);
      } else {
        if (lastTurnHistory.opponent.action === 'Reflect') {
          launchFireball(-250, 200, 2.46, -3.03, 350, 230);
        } else {
          launchFireball(-250, 200, 2.46, -3.03, 230);
        }
      }
    }
    if (opponentAttack) {
      if (lastTurnHistory.player.isDodged) {
        launchFireball(300, 200, -2.46, -3.03, 410);
      } else {
        if (lastTurnHistory.player.action === 'Reflect') {
          launchFireball(300, 200, -2.46, -3.03, 350, undefined, -180);
        } else {
          launchFireball(300, 200, -2.46, -3.03, 250);
        }
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div>
      <canvas ref={canvasRef} style={{ width: '1440px', height: '500px', position: 'absolute', top: 0, left: 0 }} />
    </div>
  );
};
