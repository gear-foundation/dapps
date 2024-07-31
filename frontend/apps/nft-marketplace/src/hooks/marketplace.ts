import { useReadWasmState, useSendMessageWithGas } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { useMemo } from 'react';
import { ADDRESS } from 'consts';
import { AuctionFormValues, MarketNFT } from 'types';
import metaTxt from 'assets/state/nft_marketplace_meta.txt';
import stateWasm from 'assets/state/market_state.meta.wasm';
import { getMilliseconds } from 'utils';
import { useBuffer, useProgramMetadata } from './metadata';

function useMarketplaceMeta() {
  return useProgramMetadata(metaTxt);
}

function useMarketplaceStateBuffer() {
  return useBuffer(stateWasm);
}

function useMarketplaceWasmState<T>(functionName: string, argument: AnyJson) {
  const programMetadata = useMarketplaceMeta();
  const buffer = useMarketplaceStateBuffer();
  
  return useReadWasmState<T>({
    programId: ADDRESS.MARKETPLACE_CONTRACT,
    wasm: buffer,
    functionName,
    payload: '0x',
    programMetadata,
    argument,
  });
}

function useMarketplace() {
  const { state, isStateRead } = useMarketplaceWasmState<MarketNFT[]>('all_items', null);

  return { NFTs: state, isEachNFTRead: isStateRead };
}

function useMarketNft(tokenId: string) {
  const payload = useMemo(() => ({ nft_contract_id: ADDRESS.NFT_CONTRACT, token_id: tokenId }), [tokenId]);

  const { state, isStateRead } = useMarketplaceWasmState<MarketNFT | null>('item_info', payload);

  return { marketNft: state, isMarketNftRead: isStateRead };
}

function useMarketplaceMessage() {
  const metadata = useMarketplaceMeta();

  return useSendMessageWithGas(ADDRESS.MARKETPLACE_CONTRACT, metadata);
}

function useMarketplaceActions(token_id: string, price: MarketNFT['price'] | undefined) {
  const sendMessage = useMarketplaceMessage();

  // eslint-disable-next-line @typescript-eslint/naming-convention
  const nft_contract_id = ADDRESS.NFT_CONTRACT;

  const buy = (onSuccess: () => void) => {
    if (!price) return;

    const payload = { BuyItem: { nft_contract_id, token_id } };
    const value = price;

    sendMessage({ payload, value, onSuccess });
  };

  const offer = (value: string, onSuccess: () => void) => {
    const payload = { AddOffer: { nft_contract_id, token_id, price: value } };

    sendMessage({ payload, value, onSuccess });
  };

  const bid = (value: string, onSuccess: () => void) => {
    const payload = { AddBid: { nft_contract_id, token_id, price: value } };

    sendMessage({ payload, value, onSuccess });
  };

  const settle = (onSuccess: () => void) => {
    const payload = { SettleAuction: { nft_contract_id, token_id } };

    sendMessage({ payload, onSuccess });
  };

  const startSale = (value: string, onSuccess: () => void) => {
    const payload = { AddMarketData: { nft_contract_id, token_id, price: value } };

    sendMessage({ payload, value, onSuccess });
  };

  const startAuction = (values: AuctionFormValues, onSuccess: () => void) => {
    const duration = getMilliseconds(values.duration);
    // eslint-disable-next-line @typescript-eslint/naming-convention
    const bid_period = getMilliseconds(values.bidPeriod);
    const { minPrice } = values;

    const marketDataPayload = { AddMarketData: { nft_contract_id, token_id } };
    const auctionPayload = { CreateAuction: { nft_contract_id, token_id, bid_period, duration, min_price: minPrice } };

    const sendStartAuctionMessage = () => sendMessage({ payload: auctionPayload, onSuccess });

    sendMessage({ payload: marketDataPayload, onSuccess: sendStartAuctionMessage });
  };

  return { buy, offer, bid, settle, startAuction, startSale };
}

export {
  useMarketplace,
  useMarketplaceStateBuffer,
  useMarketNft,
  useMarketplaceMessage,
  useMarketplaceActions,
  useMarketplaceMeta,
};
