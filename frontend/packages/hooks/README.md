# @dapps-frontend/ui

## Install

```sh
yarn add @dapps-frontend/hooks
```

## Use

### useCountdown

Count milliseconds left from provided value to current time (1 iteration per 1 second):

```jsx
import { useCountdown } from '@dapps-frontend/hooks';

type Props = {
  endTime: number,
};

function Timer({ endTime }: Props) {
  const msLeft = useCountdown(endTime);
  const secondsLeft = Math.floor(msLeft / 1000);

  return <p>Seconds Left: {secondsLeft}</p>;
}
```
