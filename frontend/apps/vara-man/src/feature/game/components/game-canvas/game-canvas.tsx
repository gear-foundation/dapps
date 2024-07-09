import React from 'react';
import BackgroundMapImg from '@/assets/images/border.png';
import MobileController from '../mobile-controller/mobile-controller';
import { GameEngine } from '../../models/Game';

type GameCanvasProps = {
  canvasRef: React.RefObject<HTMLCanvasElement>;
  fogCanvasRef: React.RefObject<HTMLCanvasElement>;
  gameInstanceRef: React.MutableRefObject<GameEngine | null>;
  isPause?: boolean;
};

export const GameCanvas = ({ canvasRef, fogCanvasRef, gameInstanceRef, isPause }: GameCanvasProps) => {
  return (
    <div
      className="ml-auto mr-auto max-md:w-full max-md:h-max z-2"
      style={{
        position: 'relative',
      }}>
      <canvas className="absolute -left-6 md:relative md:left-0 md:h-auto h-[100dvh] z-1" ref={fogCanvasRef} />
      <canvas
        ref={canvasRef}
        className="absolute -left-6 md:relative md:left-0 md:h-auto h-[100dvh]"
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
