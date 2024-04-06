export interface PlayerProps {
  onReady: (player: HTMLVideoElement) => void;
  mode: 'broadcast' | 'watch';
  isMuted?: boolean;
  onSoundMute?: (isMuted: boolean) => void;
  isCameraBlocked?: boolean;
  onCameraBlock?: (isBlocked: boolean) => void;
  onStopStream?: () => void;
  isSharingScreen?: boolean;
  onShareScreen?: (isSharingScreen: boolean) => void;
}
