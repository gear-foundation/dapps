import { Socket } from 'socket.io-client';
import { MediaStreamSequence } from '../../utils';

export interface WatchProps {
  socket: Socket;
  streamId: string;
}
export interface SectionProps {
  title: string;
  children: JSX.Element | JSX.Element[];
}

export type StreamState =
  | 'not-available'
  | 'ready-to-play'
  | 'loading'
  | 'streaming'
  | 'not-subscribed'
  | 'not-started'
  | 'ended';

export interface OfferMsg {
  userId: string;
  description: RTCSessionDescription;
  streamId: string;
  mediaSequence: MediaStreamSequence;
}

export interface CandidateMsg {
  id: string;
  candidate: RTCIceCandidate;
}

export interface ErrorMsg {
  message: string;
}
