import { useAccount } from '@gear-js/react-hooks';
import { web3FromSource } from '@polkadot/extension-dapp';
import { TransactionBuilder } from 'sails-js';

const useMakeTransaction = () => {
  const { account } = useAccount();

  return async (transactrionBuilder: TransactionBuilder<null>) => {
    if (!account?.decodedAddress) {
      throw new Error('No account found!');
    }

    const injector = await web3FromSource(account.meta.source);

    return transactrionBuilder.withAccount(account.address, { signer: injector.signer });
  };
};

export { useMakeTransaction };
