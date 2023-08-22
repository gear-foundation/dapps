import { Account } from '@gear-js/react-hooks';
import { Button, buttonStyles, TooltipWrapper } from '@gear-js/ui';
import { cn } from "app/utils";
import { SpriteIcon } from 'components/ui/sprite-icon';

type Props = {
  balance: Account['balance'];
  address: string;
  name: string | undefined;
  onClick: () => void;
};

export function WalletBalance({ balance, address, name, onClick }: Props) {
  return (
    <div className="flex gap-4 shrink-0">
      <div>
        <TooltipWrapper text="Account gas balance">
          <Button
            className={cn('group !p-2.5', buttonStyles.lightGreen)}
            icon={() => (
              <>
                <SpriteIcon name="test-balance" width={20} height={20} />
                <SpriteIcon
                  name="plus"
                  width={12}
                  height={12}
                  className="absolute bottom-2 right-1.5 bg-[#223428] group-hover:bg-[#285b3a] rounded-full transition-colors"
                />
              </>
            )}
          />
        </TooltipWrapper>
      </div>
      <p className="shrink-0 grid grid-cols-[auto_auto] gap-x-1 font-kanit">
        <span className="col-span-2 text-[10px] text-white text-opacity-80">Gas Balance:</span>
        <span className="font-medium text-lg leading-none">{balance.value}</span>
        <span className="text-sm text-white text-opacity-70">{balance.unit}</span>
      </p>
    </div>
  );
}
