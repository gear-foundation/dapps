import { Moment } from 'moment';
import { TimePickerProps as RCTimePickerProps } from 'rc-time-picker';

export interface TimePickerProps extends RCTimePickerProps {
  onChange?: (time: Moment) => void;
}
