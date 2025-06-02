import { useAlert } from '@gear-js/react-hooks';
import { useForm, isNotEmpty } from '@mantine/form';
import { useAtom } from 'jotai';
import moment, { Moment } from 'moment';
import { useCallback } from 'react';
import { useNavigate } from 'react-router-dom';

import { STREAMS } from '@/App.routes';
import { useGetStateQuery, useNewStreamMessage } from '@/app/utils';
import CreateSVG from '@/assets/icons/correct-icon.svg';
import { Button, Calendar, TextField, InputArea } from '@/ui';
import { TimePicker } from '@/ui/TimePicker';
import { cx } from '@/utils';

import { IS_CREATING_STREAM } from '../../atoms';
import { PictureDropzone } from '../PictureDropzone';

import { FormValues, SectionProps } from './LayoutCreateForm.interface';
import styles from './LayoutCreateForm.module.scss';

function Section({ title, children }: SectionProps) {
  return (
    <div>
      <h5 className={cx(styles['section-label'])}>{title}</h5>
      {children}
    </div>
  );
}

function LayoutCreateForm() {
  const { newStreamMessage } = useNewStreamMessage();
  const alert = useAlert();
  const navigate = useNavigate();
  const [isCreatingStream, setIsCreatingStream] = useAtom(IS_CREATING_STREAM);
  const { refetch } = useGetStateQuery();

  const form = useForm({
    initialValues: {
      title: '',
      description: '',
      dayDate: new Date(),
      startTime: moment(),
      endTime: moment(),
      imgLink: '',
    },
    validate: {
      title: isNotEmpty('Stream title is required'),
      startTime: (value, values) => (value.isSame(values.endTime) ? 'Start time shouldnt be equal to End time' : null),
      endTime: (value, values) =>
        value.isBefore(values.startTime) ? `Start time shouldn't be less than End time` : null,
    },
  });

  const { errors, getInputProps, setFieldValue, onSubmit, reset, values } = form;

  const handleChangeDate = (field: 'dayDate' | 'startTime' | 'endTime', value: Moment | Date) => {
    setFieldValue(field, value);
  };

  const handleTransformData = ({ title, description, dayDate, startTime, endTime, imgLink }: FormValues) => {
    const day = dayDate.getDate();
    const month = dayDate.getMonth();
    const year = dayDate.getFullYear();

    const startDate = new Date(year, month, day);
    startDate.setHours(startTime.hour());
    startDate.setMinutes(startTime.minute());
    startDate.setSeconds(0);
    const startTimestamp = startDate.getTime();

    const endDate = new Date(year, month, day);
    endDate.setHours(endTime.hour());
    endDate.setMinutes(endTime.minute());
    endDate.setSeconds(0);
    const endTimestamp = endDate.getTime();

    setIsCreatingStream(true);

    newStreamMessage(
      {
        startTime: startTimestamp,
        endTime: endTimestamp,
        title: title.trim(),
        description: description?.trim() || null,
        imgLink,
      },
      {
        onSuccess: () => {
          reset();
          refetch();
          alert.success('Stream has been successfully created');
          navigate(`/${STREAMS}`);
        },
      },
    );
  };

  const handleDropImg = (preview: string[]) => {
    setFieldValue('imgLink', preview[0]);
  };

  const handleTimePickerDisabledHours = useCallback(() => {
    const now = moment();
    if (moment(values.dayDate).isSame(now, 'day')) {
      const end = now.hour();
      return [...Array(end).keys()];
    }
    return [];
  }, [values.dayDate]);

  const handleTimePickerDisabledMinutes = useCallback(
    (hour: number) => {
      const now = moment();
      if (hour === moment().hour() && moment(values.dayDate).isSame(now, 'day')) {
        const end = moment().minutes();
        return [...Array(end).keys()];
      }

      return [];
    },
    [values.dayDate],
  );

  return (
    <div className={cx(styles.layout)}>
      <h1 className={cx(styles.title)}>Create stream</h1>
      <form onSubmit={onSubmit(handleTransformData)}>
        <div className={cx(styles.content)}>
          <div className={cx(styles.left)}>
            <div className={cx(styles['dropzone-wrapper'])}>
              <PictureDropzone
                onDropFile={handleDropImg}
                previewLinks={getInputProps('imgLink').value ? [getInputProps('imgLink').value] : undefined}
              />
            </div>
            <Section title="Stream info">
              <div className={cx(styles.inputs)}>
                <div className={cx(styles.input)}>
                  <TextField
                    size="large"
                    label="Stream title"
                    {...getInputProps('title')}
                    disabled={isCreatingStream}
                  />
                  <span className={cx(styles['field-error'])}>{errors.title}</span>
                </div>
                <div className={cx(styles.input)}>
                  <InputArea
                    placeholder="Stream description"
                    {...getInputProps('description')}
                    disabled={isCreatingStream}
                  />
                </div>
                <div className={cx(styles.controls)}>
                  <Button
                    variant="primary"
                    label="Create"
                    icon={CreateSVG}
                    size="large"
                    type="submit"
                    isLoading={isCreatingStream}
                  />
                  {/* <Button variant="text" label="Cancel" icon={CrossSVG} size="large" /> */}
                </div>
              </div>
            </Section>
          </div>
          <div className={cx(styles.right)}>
            <Section title="Stream date">
              <div className={cx(styles['datepicker-wrapper'])}>
                <Calendar onChange={(day: Date) => handleChangeDate('dayDate', day)} />
              </div>
            </Section>
            <Section title="Stream time">
              <div className={cx(styles['time-pickers-wrapper'])}>
                <TimePicker
                  disabledHours={handleTimePickerDisabledHours}
                  disabledMinutes={handleTimePickerDisabledMinutes}
                  onChange={(time: Moment) => handleChangeDate('startTime', time)}
                  disabled={isCreatingStream}
                />
                -
                <TimePicker
                  disabledHours={handleTimePickerDisabledHours}
                  disabledMinutes={handleTimePickerDisabledMinutes}
                  onChange={(time: Moment) => handleChangeDate('endTime', time)}
                  disabled={isCreatingStream}
                />
              </div>
              <span className={cx(styles['field-error'])}>{errors.startTime}</span>
              <span className={cx(styles['field-error'])}>{errors.endTime}</span>
            </Section>
          </div>
        </div>
      </form>
    </div>
  );
}

export { LayoutCreateForm };
