import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useState } from 'react';

import { useTokenActions, useTokenQueries, useTokenEvents, useBalanceOfQuery } from '../../hooks';

import { styles } from './styles';
import { toActorId } from './helper';


function isZeroAddress(address: string) {
  return address === '0x0000000000000000000000000000000000000000';
}

function Home() {
  const { account } = useAccount();
  const { name, symbol, decimals, totalSupply, isLoading, refetchTotalSupply } = useTokenQueries();
  const { mint, burn, mintPending, burnPending, transfer, transferPending } = useTokenActions();
  const alert = useAlert();

  const [transferTo, setTransferTo] = useState('');
  const [transferValue, setTransferValue] = useState('');

  // Balance Of logic
  const [balanceAddr, setBalanceAddr] = useState('');
  const [checkedAddr, setCheckedAddr] = useState('');
  const [showBalance, setShowBalance] = useState(false);

  useTokenEvents({
    onMinted: () => void refetchTotalSupply?.(),
    onBurned: () => void refetchTotalSupply?.(),
  });

  const handleMint = async () => {
    try {
      if (!account?.address) return alert.error('No account selected!');
      await mint(toActorId(account.address), '1000');
      alert.success('Mint success!');
    } catch (e) {
      alert.error('Error mint');
      console.error(e);
    }
  };

  const handleBurn = async () => {
    try {
      if (!account?.address) return alert.error('No account selected!');
      await burn(toActorId(account.address), '1000');
      alert.success('Burn success!');
    } catch (e) {
      alert.error('Error burn');
      console.error(e);
    }
  };

  const handleTransfer = async () => {
    try {
      if (!account?.address) return alert.error('No account selected!');
      await transfer(toActorId(transferTo), transferValue);
      alert.success('Transfer success!');
      setTransferValue('');
      setTransferTo('');
    } catch (e) {
      alert.error('Error transfer');
      console.error(e);
    }
  };

  const handleBalanceOf = () => {
    setCheckedAddr(balanceAddr.trim());
    setShowBalance(true);
  };

  const actorId = toActorId(checkedAddr);
  const balanceQuery = useBalanceOfQuery(actorId);

  return (
    <div style={styles.container}>
      <div style={{ marginBottom: 24, textAlign: 'center' }}>
        <div style={{ fontSize: 24, fontWeight: 700 }}>Token Metadata</div>
        {isLoading ? (
          <div style={{ fontSize: 18 }}>Loading token data...</div>
        ) : (
          <>
            <div><strong>Name:</strong> {name}</div>
            <div><strong>Symbol:</strong> {symbol}</div>
            <div><strong>Decimals:</strong> {decimals}</div>
            <div><strong>Total Supply:</strong> {totalSupply}</div>
          </>
        )}
        <div style={{ marginTop: 12, color: '#6c6c6c', fontSize: 14 }}>
          <strong>Your Account:</strong> {account?.address ?? 'Not connected'}
        </div>
      </div>

      <button
        onClick={handleMint}
        disabled={mintPending || !account?.address}
        style={{ ...styles.button, fontSize: 20, padding: '14px 30px', marginBottom: 12 }}>
        {mintPending ? 'Minting...' : 'Mint 1000 to self'}
      </button>
      <button
        onClick={handleBurn}
        disabled={burnPending || !account?.address}
        style={{ ...styles.button, fontSize: 20, padding: '14px 30px', marginBottom: 24 }}>
        {burnPending ? 'Burning...' : 'Burn 1000 from self'}
      </button>

      <div style={styles.section}>
        <div style={styles.label}>Transfer</div>
        <input
          placeholder="To address"
          value={transferTo}
          onChange={e => setTransferTo(e.target.value)}
          style={styles.input}
        />
        <input
          placeholder="Amount"
          value={transferValue}
          onChange={e => setTransferValue(e.target.value)}
          style={styles.input}
        />
        <button
          onClick={handleTransfer}
          disabled={transferPending || !transferTo || !transferValue}
          style={styles.button}>
          {transferPending ? 'Transferring...' : 'Transfer'}
        </button>
      </div>

      <div style={styles.section}>
        <div style={styles.label}>Balance Of</div>
        <input
          placeholder="Address"
          value={balanceAddr}
          onChange={e => setBalanceAddr(e.target.value)}
          style={styles.input}
        />
        <button
          onClick={handleBalanceOf}
          style={styles.button}>
          Check Balance
        </button>
        {showBalance && checkedAddr && (
          isZeroAddress(actorId)
            ? <div style={styles.error}>Balance: <strong>Error</strong></div>
            : <div style={styles.balance}>Balance: <strong>{balanceQuery.data !== undefined ? balanceQuery.data?.toString() : ''}</strong></div>
        )}
      </div>
    </div>
  );
}

export { Home };