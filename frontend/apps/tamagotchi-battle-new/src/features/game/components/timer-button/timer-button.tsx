import { Button, ButtonProps } from '@gear-js/vara-ui';
import { UseTimerParams, useTimer } from '../../hooks';

type Props = UseTimerParams &
  ButtonProps & {
    isYourTurn?: boolean;
  };

export function TimerButton({ remainingTime, shouldGoOn, isYourTurn, text, children, ...restProps }: Props) {
  const formattedTimeLeft = useTimer({ remainingTime, shouldGoOn });

  return <Button color="primary" {...restProps} text={`${text} (${formattedTimeLeft})`} />;
}
