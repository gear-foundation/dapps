export type MediaTrackSequenceType = 'microphone' | 'camera' | 'screenCapture' | 'screenSound';

export type TrackIds = Record<MediaTrackSequenceType, string | null>;
