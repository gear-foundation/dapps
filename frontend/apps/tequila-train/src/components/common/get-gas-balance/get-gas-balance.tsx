import { buttonStyles } from '@gear-js/ui';
import { Icon } from 'components/ui/icon';
import clsx from 'clsx';

export const GetGasBalance = () => {
  return (
    <div className="">
      <button className={clsx('btn group !p-2.5', buttonStyles.lightGreen)}>
        <Icon name="test-balance" width={20} height={20} />
      </button>
    </div>
  );
};
