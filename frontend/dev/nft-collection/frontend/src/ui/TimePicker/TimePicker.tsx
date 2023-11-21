import { useState } from 'react';
import moment, { Moment } from 'moment';
import ReactTimePicker from 'rc-time-picker';
import { cx } from '@/utils';
import { TimePickerProps } from './TimePicker.interface';
import SelectArrowSVG from '@/assets/icons/select-arrow.svg';
import styles from './TimePicker.module.scss';

import 'rc-time-picker/assets/index.css';

function TimePicker({ onChange, ...props }: TimePickerProps) {
  const [value, setValue] = useState<Moment>(moment());

  const handleChangeTime = (val: Moment) => {
    setValue(val);
    onChange?.(val);
  };

  return (
    <div className={cx(styles.container)}>
      <ReactTimePicker
        format="h:mm A"
        showSecond={false}
        value={value}
        clearIcon={<></>}
        placement="bottomLeft"
        minuteStep={5}
        onChange={handleChangeTime}
        inputIcon={<img src={SelectArrowSVG} alt="select time" className={cx(styles.select)} />}
        className={cx(styles.input)}
        popupClassName={cx(styles.popup)}
        {...props}
      />
    </div>
  );
}

export { TimePicker };
