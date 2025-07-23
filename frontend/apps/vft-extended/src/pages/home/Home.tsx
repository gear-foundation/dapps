import { useAccount, useAlert } from '@gear-js/react-hooks';
import { Button, Input } from '@gear-js/vara-ui';
import { useState } from 'react';

import {
  useSendMintTransaction,
  useSendBurnTransaction,
  useSendTransferTransaction,
  useTokenQueries,
  useTokenEvents,
  useBalanceOfQuery,
} from '../../hooks';

import styles from './Home.module.scss';
import { isValidAddress } from './helper';

function Home() {
  const { account } = useAccount();
  const { name, symbol, decimals, totalSupply, isLoading, refetchTotalSupply } = useTokenQueries();
  const alert = useAlert();

  const [transferTo, setTransferTo] = useState('');
  const [transferValue, setTransferValue] = useState('');
  const [balanceAddr, setBalanceAddr] = useState('');

  const { sendTransactionAsync: sendMint, isPending: mintPending } = useSendMintTransaction();
  const { sendTransactionAsync: sendBurn, isPending: burnPending } = useSendBurnTransaction();
  const { sendTransactionAsync: sendTransfer, isPending: transferPending } = useSendTransferTransaction();

  useTokenEvents({
    onMinted: (data) => alert.info(`Mint event: ${JSON.stringify(data)}`),
    onBurned: (data) => alert.info(`Burn event: ${JSON.stringify(data)}`),
    onTransfer: (data) => alert.info(`Transfer event: ${JSON.stringify(data)}`),
    onApproval: (data) => alert.info(`Approval event: ${JSON.stringify(data)}`),
  });

  const handleMint = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (!account?.address) throw new Error('No account selected!');
      await sendMint({ args: [account.decodedAddress, '1000'] });
      alert.success('Mint success!');
    } catch (error) {
      alert.error('Error mint');
      console.error(error);
    }
  };

  const handleBurn = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (!account?.address) throw new Error('No account selected!');
      await sendBurn({ args: [account.decodedAddress, '1000'] });
      await refetchTotalSupply?.();
      alert.success('Burn success!');
    } catch (error) {
      alert.error('Error burn');
      console.error(error);
    }
  };

  const handleTransfer = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!account?.address) throw new Error('No account selected!');
    if (!isValidAddress(transferTo)) throw new Error('Invalid recipient address!');
    try {
      await sendTransfer({ args: [transferTo as `0x${string}`, transferValue] });
      alert.success('Transfer success!');
      setTransferValue('');
      setTransferTo('');
    } catch (error) {
      alert.error('Error transfer');
      console.error(error);
    }
  };

  const balanceQuery = isValidAddress(balanceAddr) ? useBalanceOfQuery(balanceAddr as `0x{string}`) : null;

  return (
    <main className={styles.container}>
      <header className={styles.header}>
        <h1 className={styles.title}>Token Metadata</h1>
        {isLoading ? (
          <p className={styles.loading}>Loading token data...</p>
        ) : (
          <dl className={styles.metaList}>
            <div>
              <dt>Name:</dt>
              <dd>{name}</dd>
            </div>
            <div>
              <dt>Symbol:</dt>
              <dd>{symbol}</dd>
            </div>
            <div>
              <dt>Decimals:</dt>
              <dd>{decimals}</dd>
            </div>
            <div>
              <dt>Total Supply:</dt>
              <dd>{totalSupply}</dd>
            </div>
          </dl>
        )}
        <div className={styles.account}>
          <strong>Your Account:</strong> {account?.address ?? 'Not connected'}
        </div>
      </header>

      <form className={styles.section} onSubmit={handleMint} autoComplete="off">
        <Button type="submit" color="primary" size="medium" isLoading={mintPending} disabled={!account?.address} block>
          {mintPending ? 'Minting...' : 'Mint 1000 to self'}
        </Button>
      </form>

      <form className={styles.section} onSubmit={handleBurn} autoComplete="off">
        <Button type="submit" color="contrast" size="medium" isLoading={burnPending} disabled={!account?.address} block>
          {burnPending ? 'Burning...' : 'Burn 1000 from self'}
        </Button>
      </form>

      <form className={styles.section} onSubmit={handleTransfer} autoComplete="off">
        <Input
          label="To address"
          value={transferTo}
          onChange={(e) => setTransferTo(e.target.value)}
          placeholder="To address"
          size="medium"
          block
          error={transferTo && !isValidAddress(transferTo) ? 'Incorrect address' : undefined}
        />
        <Input
          label="Amount"
          type="number"
          value={transferValue}
          onChange={(e) => setTransferValue(e.target.value)}
          placeholder="Amount"
          size="medium"
          min="0"
          block
        />
        <Button
          type="submit"
          color="primary"
          size="medium"
          isLoading={transferPending}
          disabled={transferPending || !transferTo || !transferValue || !isValidAddress(transferTo)}
          block>
          {transferPending ? 'Transferring...' : 'Transfer'}
        </Button>
      </form>

      <form className={styles.section} autoComplete="off">
        <Input
          label="Address"
          value={balanceAddr}
          onChange={(e) => setBalanceAddr(e.target.value)}
          placeholder="Address"
          size="medium"
          block
          error={balanceAddr && !isValidAddress(balanceAddr) ? 'Incorrect address' : undefined}
        />
        <Button
          type="button"
          color="primary"
          size="medium"
          disabled={!isValidAddress(balanceAddr)}
          block
          onClick={() => setBalanceAddr(balanceAddr.trim())}>
          Check Balance
        </Button>
        {isValidAddress(balanceAddr) && (
          <div className={styles.balance}>
            Balance: <strong>{balanceQuery?.data !== undefined ? balanceQuery.data?.toString() : ''}</strong>
          </div>
        )}
      </form>
    </main>
  );
}

export { Home };
