export type TokenMetaQueries = {
  name?: string;
  symbol?: string;
  decimals?: number;
  totalSupply?: bigint | string;
  isLoading: boolean;
  refetchTotalSupply?: () => Promise<unknown>;
};

type MintedEvent = { to: string; value: string | number | bigint };
type BurnedEvent = { from: string; value: string | number | bigint };
type ApprovalEvent = { owner: string; spender: string; value: string | number | bigint };
type TransferEvent = { from: string; to: string; value: string | number | bigint };

export type TokenEventCallbacks = Partial<{
  onMinted: (event: MintedEvent) => void;
  onBurned: (event: BurnedEvent) => void;
  onApproval: (event: ApprovalEvent) => void;
  onTransfer: (event: TransferEvent) => void;
}>;
