import { withoutCommas } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { Content } from 'components';
import { useLotteryMessage } from 'hooks';

type Props = {
  cost: string;
  isToken: boolean;
};

function PlayerStart({ cost, isToken }: Props) {
  const sendMessage = useLotteryMessage();

  const subheading = `Cost of participation is ${cost}. This amount will be withdrawn from your balance. Click "Enter" if you want to proceed.`;

  const enter = () => {
    sendMessage(
      { Enter: null },
      isToken ? undefined : { value: withoutCommas(cost) }
    );
  };

  return (
    <Content subheading={subheading}>
      <Button text="Enter" onClick={enter} />
    </Content>
  );
}

export { PlayerStart };
