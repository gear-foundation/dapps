export interface SubscribeModalProps {
  onClose: () => void;
  speakerId?: string | null;
  type: 'subscribe' | 'unsubscribe';
}
