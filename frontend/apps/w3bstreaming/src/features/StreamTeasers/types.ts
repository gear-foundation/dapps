import { Profile, Stream } from '@/app/utils';

export interface StreamWithInfo extends Stream {
  broadcasterInfo?: Profile;
}

export type StreamProps = StreamWithInfo;

export interface Streams {
  [key: string]: Stream;
}

export interface FormattedTeaser extends Stream {
  id: string;
}
