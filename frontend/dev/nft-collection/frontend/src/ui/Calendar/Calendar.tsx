import { useRef, useState } from 'react';
import DatePicker, { ReactDatePickerCustomHeaderProps } from 'react-datepicker';
import { Button } from '@/ui';
import { cx } from '@/utils';
import PlaySVG from '@/assets/icons/play-icon.svg';
import chevronLeftSVG from '@/assets/icons/chevron-left.svg';
import chevronRightSVG from '@/assets/icons/chevron-right.svg';
import { CalendarProps } from './Calendar.interfaces';
import styles from './Calendar.module.scss';

import 'react-datepicker/dist/react-datepicker.css';

function Calendar({ onChange }: CalendarProps) {
  const [selectedDate, setSelectedDate] = useState<Date>(new Date());
  const calendarRef = useRef<DatePicker<never, undefined>>(null);

  const handleChangeCalendar = (value: Date) => {
    setSelectedDate(value);
    onChange?.(value);
  };

  const areDaysEqaual = (date1: Date, date2: Date) =>
    date1.getFullYear() === date2.getFullYear() &&
    date1.getMonth() === date2.getMonth() &&
    date1.getDate() === date2.getDate();

  const handleRenderCustomHeader = ({
    monthDate,
    decreaseMonth,
    increaseMonth,
    changeMonth,
  }: ReactDatePickerCustomHeaderProps): JSX.Element => {
    const onPrevMonth = () => {
      decreaseMonth();
    };

    const onNextMonth = () => {
      increaseMonth();
    };

    const goToCurrentDate = () => {
      const today = new Date();

      setSelectedDate(today);
      changeMonth(today.getMonth());
    };

    return (
      <>
        <div className={cx(styles['calendar-date-day'])}>
          <button className={cx(styles['calendar-date-day-name'])} onClick={goToCurrentDate}>
            Today
          </button>
          <div className={cx(styles['calendar-date-day-value'])}>
            <img src={PlaySVG} alt="play" />
            <span>
              {selectedDate.toLocaleDateString('default', {
                day: '2-digit',
                month: 'short',
                year: 'numeric',
              })}
            </span>
          </div>
        </div>
        <div className={cx(styles['calendar-header'])}>
          <span className={cx(styles['calendar-header-month'])}>
            {monthDate.toLocaleDateString('en-US', {
              month: 'short',
              year: 'numeric',
            })}
          </span>
          <div className={cx(styles['calendar-header-controls'])}>
            <Button variant="icon" label="" icon={chevronLeftSVG} onClick={onPrevMonth} />
            <Button variant="icon" label="" icon={chevronRightSVG} onClick={onNextMonth} />
          </div>
        </div>
      </>
    );
  };

  const handleDayClassname = (date: Date) => {
    if (areDaysEqaual(date, selectedDate)) {
      return cx(styles['calendar-day'], styles['calendar-day-selected']);
    }

    return cx(styles['calendar-day']);
  };

  const handleWeekDayClassname = () => cx(styles['calendar-week-day']);

  const handleDisableTiles = (date: Date): boolean => {
    const now = new Date();
    now.setHours(0, 0, 0, 0);
    date.setHours(0, 0, 0, 0);

    return date.getTime() >= now.getTime();
  };

  return (
    <div className={cx(styles['calendar-container'])}>
      <DatePicker
        inline
        fixedHeight
        disabledKeyboardNavigation
        showPopperArrow={false}
        selected={selectedDate}
        formatWeekDay={(day) => day.substring(0, 3)}
        onChange={handleChangeCalendar}
        renderCustomHeader={handleRenderCustomHeader}
        dayClassName={handleDayClassname}
        weekDayClassName={handleWeekDayClassname}
        filterDate={handleDisableTiles}
        ref={calendarRef}>
        <div />
      </DatePicker>
    </div>
  );
}

export { Calendar };
