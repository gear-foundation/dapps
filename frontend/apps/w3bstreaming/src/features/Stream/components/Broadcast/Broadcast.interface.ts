import { Socket } from 'socket.io-client';

export interface SectionProps {
  title: string;
  children: JSX.Element | JSX.Element[];
}

export interface BroadcastProps {
  socket: Socket;
  streamId: string;
}

export interface WatchMsg {
  streamId: string;
  signedMsg: string;
}

export interface AnswerMsg {
  userId: string;
  streamId: string;
  description: RTCSessionDescription;
}

export interface CandidateMsg {
  id: string;
  candidate: RTCIceCandidate;
}

export type StreamStatus = 'not-started' | 'streaming' | 'ended' | 'already-have' | 'loading';

export type StreamType = 'camera' | 'screen';
