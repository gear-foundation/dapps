import { ProgramMetadata } from '@gear-js/api';
import { Moment } from 'moment';

export interface SectionProps {
  title: string;
  children: JSX.Element | JSX.Element[];
}

export interface FormValues {
  title: string;
  description?: string;
  dayDate: Date;
  startTime: Moment;
  endTime: Moment;
  imgLink: string;
}

export interface LayoutCreateFormProps {
  meta?: ProgramMetadata;
}
