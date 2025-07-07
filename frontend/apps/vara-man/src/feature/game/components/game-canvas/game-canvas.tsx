import React, { useEffect } from 'react';

import BackgroundMapImg from '@/assets/images/border.png';

import { GameEngine } from '../../models/Game';
import MobileController from '../mobile-controller/mobile-controller';

type GameCanvasProps = {
  canvasRef: React.RefObject<HTMLCanvasElement>;
  fogCanvasRef: React.RefObject<HTMLCanvasElement>;
  gameInstanceRef: React.RefObject<GameEngine | null>;
  isPause?: boolean;
};

const useResizeCanvas = (
  canvasRef: GameCanvasProps['canvasRef'],
  fogCanvasRef: GameCanvasProps['fogCanvasRef'],
  gameInstanceRef: GameCanvasProps['gameInstanceRef'],
) => {
  useEffect(() => {
    const resizeCanvas = () => {
      const canvas = canvasRef.current;
      const fogCanvas = fogCanvasRef.current;
      if (canvas && fogCanvas) {
        const dpr = window.devicePixelRatio;
        canvas.width = canvas.clientWidth * dpr;
        canvas.height = canvas.clientHeight * dpr;
        fogCanvas.width = fogCanvas.clientWidth * dpr;
        fogCanvas.height = fogCanvas.clientHeight * dpr;
        const ctx = canvas.getContext('2d');
        const fogCtx = fogCanvas.getContext('2d');
        if (ctx) ctx.scale(dpr, dpr);
        if (fogCtx) fogCtx.scale(dpr, dpr);
        if (gameInstanceRef.current) {
          gameInstanceRef.current.resize();
        }
      }
    };

    if (gameInstanceRef.current) {
      resizeCanvas();
    } else {
      const timeoutId = setTimeout(resizeCanvas, 100);
      return () => clearTimeout(timeoutId);
    }

    window.addEventListener('resize', resizeCanvas);

    return () => {
      window.removeEventListener('resize', resizeCanvas);
    };
  }, [canvasRef, fogCanvasRef, gameInstanceRef.current]);
};

export const GameCanvas = ({ canvasRef, fogCanvasRef, gameInstanceRef, isPause }: GameCanvasProps) => {
  useResizeCanvas(canvasRef, fogCanvasRef, gameInstanceRef);

  return (
    <div className="ml-auto mr-auto max-md:w-full max-md:h-max z-2 h-screen lg:h-full" style={{ position: 'relative' }}>
      <canvas
        className="-left-6 md:relative md:left-0 md:h-auto  z-1"
        style={{ position: 'absolute' }}
        ref={fogCanvasRef}
      />
      <canvas
        ref={canvasRef}
        className="absolute -left-6 md:relative md:left-0 "
        style={{
          backgroundImage: `radial-gradient(circle, rgba(255,255,255,0) 25%, rgba(255,255,255,1) 65%), url(${BackgroundMapImg})`,
          backgroundRepeat: 'no-repeat',
          backgroundPosition: 'center',
        }}
      />
      {!isPause && <MobileController gameInstanceRef={gameInstanceRef} />}
    </div>
  );
};
