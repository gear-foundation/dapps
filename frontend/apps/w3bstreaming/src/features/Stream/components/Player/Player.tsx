import { ChangeEvent, MutableRefObject, useEffect, useRef, useState } from 'react';
import { PlayerProps } from './Player.interfaces';
import { cx } from '@/utils';
import styles from './Player.module.scss';
import { Button } from '@/ui';
import PlaySVG from '@/assets/icons/player-play-icon.svg';
import MicSVG from '@/assets/icons/player-mic-icon.svg';
import MutedMicSVG from '@/assets/icons/player-mic-muted-icon.svg';
import ShareSVG from '@/assets/icons/player-share-icon.svg';
import ShareActiveSVG from '@/assets/icons/player-share-active-icon.svg';
import CameraSVG from '@/assets/icons/player-camera-icon.svg';
import BLockedCameraSVG from '@/assets/icons/player-camera-blocked-icon.svg';
import LeaveSVG from '@/assets/icons/player-leave-icon.svg';
import FullScreenSVG from '@/assets/icons/player-fullscreen-icon.svg';
import VolumeSVG from '@/assets/icons/player-volume-icon.svg';
import VolumeMutedSVG from '@/assets/icons/player-volume-muted-icon.svg';
import PauseSVG from '@/assets/icons/pause-icon.svg';

function Player({
  mode,
  onReady,
  isMuted = false,
  onSoundMute,
  isCameraBlocked = false,
  onCameraBlock,
  onStopStream,
  isSharingScreen = false,
  onShareScreen,
}: PlayerProps) {
  const playerRef: MutableRefObject<HTMLVideoElement | null> = useRef(null);
  const playerContainer: MutableRefObject<HTMLDivElement | null> = useRef(null);
  const prevVolume: MutableRefObject<number> = useRef(0);
  const [isOnPause, setIsOnPause] = useState<boolean>(false);
  const [volume, setVolume] = useState(50);

  useEffect(() => {
    playerRef.current?.load();
    onReady?.(playerRef.current as HTMLVideoElement);
  }, [onReady]);

  useEffect(() => {
    const player = playerRef.current;
    player!.volume = volume / 100;
  }, [volume]);

  const handlePause = async () => {
    if (isOnPause && playerRef.current?.paused) {
      playerRef.current?.play();
      setIsOnPause(false);
    } else {
      playerRef.current?.pause();
      setIsOnPause(true);
    }
  };

  const handleVolumeChange = (e: ChangeEvent<HTMLInputElement>) => {
    const volumePercent = e.target.value;
    setVolume(Number(volumePercent));
  };

  const handleMuteVolume = () => {
    if (volume) {
      prevVolume.current = volume;
      setVolume(() => 0);
    } else {
      setVolume(() => prevVolume.current);
    }
  };

  const handleFullScreen = () => {
    if (document.fullscreenElement) {
      document.exitFullscreen();
    } else {
      playerContainer.current?.requestFullscreen();
    }
  };

  const handleOnPause = () => {
    setIsOnPause(true);
  };

  const handleOnPlaying = () => {
    setIsOnPause(false);
  };

  return (
    <div className={cx(styles['player-container'])} ref={playerContainer}>
      <video
        className={cx(styles.player)}
        controls={false}
        preload="auto"
        muted={mode === 'broadcast'}
        onPause={handleOnPause}
        onPlaying={handleOnPlaying}
        ref={playerRef}
        id="audio"
        playsInline
        autoPlay>
        <track kind="captions" src="captions.vtt" label="English" />
      </video>
      <div className={cx(styles.controls)}>
        <div className={cx(styles.left, styles.part)}>
          {mode === 'watch' && (
            <div className={cx(styles.volume)}>
              <Button variant="icon" label="" icon={volume ? VolumeSVG : VolumeMutedSVG} onClick={handleMuteVolume} />
              <input type="range" min="0" max="100" onChange={handleVolumeChange} value={volume} />
            </div>
          )}
        </div>
        <div className={cx(styles.center, styles.part)}>
          {mode === 'watch' && (
            <Button variant="icon" label="" icon={isOnPause ? PlaySVG : PauseSVG} onClick={handlePause} />
          )}
          {mode === 'broadcast' && (
            <Button
              variant="icon"
              label=""
              icon={isMuted ? MutedMicSVG : MicSVG}
              onClick={() => onSoundMute?.(isMuted)}
            />
          )}
          {mode === 'broadcast' && (
            <Button
              variant="icon"
              label=""
              icon={isSharingScreen ? ShareActiveSVG : ShareSVG}
              onClick={() => onShareScreen?.(isSharingScreen)}
            />
          )}
          {mode === 'broadcast' && (
            <Button
              variant="icon"
              label=""
              icon={isCameraBlocked ? BLockedCameraSVG : CameraSVG}
              onClick={() => onCameraBlock?.(isCameraBlocked)}
            />
          )}
          {mode === 'broadcast' && <Button variant="icon" label="" icon={LeaveSVG} onClick={onStopStream} />}
        </div>
        <div className={cx(styles.right, styles.part)}>
          <Button variant="icon" label="" icon={FullScreenSVG} onClick={handleFullScreen} />
        </div>
      </div>
    </div>
  );
}

export { Player };
