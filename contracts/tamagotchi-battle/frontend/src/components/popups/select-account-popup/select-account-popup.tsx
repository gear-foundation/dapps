import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { Modal } from '@gear-js/ui';
import { AccountsList } from 'components/common/accounts-list';
import { ScrollWrapper } from '../../common/scroll-wrapper/scroll-wrapper';

type Props = {
  accounts: InjectedAccountWithMeta[] | undefined;
  close: () => void;
};

export const SelectAccountPopup = ({ accounts, close }: Props) => (
  <Modal heading="Connect" close={close}>
    {accounts ? (
      <div className="flex flex-col grow bg-dark-500 -m-8 p-8 rounded-b-[20px]">
        <ScrollWrapper>
          <AccountsList list={accounts} onChange={close} />
        </ScrollWrapper>
      </div>
    ) : (
      <p>
        Polkadot extension was not found or disabled. Please,{' '}
        <a href="https://polkadot.js.org/extension/" target="_blank" rel="noreferrer">
          install it
        </a>
        .
      </p>
    )}
  </Modal>
);
