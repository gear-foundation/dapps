import { AuctionFormValues, MarketNFT } from '@/types';
import { getMilliseconds } from '@/utils';
import {
  useBuyItemMessage,
  useAddOfferMessage,
  useAddBidMessage,
  useSettleAuctionMessage,
  useAddMarketDataMessage,
  useCreateAuctionMessage,
  useGetMarketQuery,
} from '@/app/utils';
import { useApproveMessage } from '@/app/utils/sails/messages/use-approve-message';
import { useAlert } from '@gear-js/react-hooks';

function useMarketplaceActions(tokenId: string, price: MarketNFT['price'] | undefined, isMarketOwner: boolean) {
  const alert = useAlert();
  const { refetch } = useGetMarketQuery();
  const { buyItemMessage } = useBuyItemMessage();
  const { addOfferMessage } = useAddOfferMessage();
  const { addBidMessage } = useAddBidMessage();
  const { settleAuctionMessage } = useSettleAuctionMessage();
  const { addMarketDataMessage } = useAddMarketDataMessage();
  const { createAuctionMessage } = useCreateAuctionMessage();
  const { approveMessage } = useApproveMessage();

  const buy = (onSuccess: () => void) => {
    if (!price) return;

    buyItemMessage(
      { tokenId, value: BigInt(price) },
      {
        onSuccess: () => {
          onSuccess();
          refetch();
        },
      },
    );
  };

  const offer = (value: string, onSuccess: () => void) => {
    addOfferMessage(
      { tokenId, price: BigInt(value), value: BigInt(value) },
      {
        onSuccess: () => {
          onSuccess();
          refetch();
        },
      },
    );
  };

  const bid = (value: string, onSuccess: () => void) => {
    addBidMessage(
      { tokenId, price: BigInt(value), value: BigInt(value) },
      {
        onSuccess: () => {
          onSuccess();
          refetch();
        },
      },
    );
  };

  const settle = (onSuccess: () => void) => {
    settleAuctionMessage({ tokenId }, { onSuccess });
  };

  const startSale = (value: string, onSuccess: () => void) => {
    const addMarketData = () => {
      addMarketDataMessage(
        { tokenId, price: BigInt(value), value: BigInt(value) },
        {
          onSuccess: () => {
            onSuccess();
            alert.success('Sale started');
          },
        },
      );
    };
    if (isMarketOwner) {
      addMarketData();
    } else {
      approveMessage(
        { tokenId },
        {
          onSuccess: () => {
            alert.info('NFT approved');
            addMarketData();
          },
        },
      );
    }
  };

  const startAuction = (values: AuctionFormValues, onSuccess: () => void) => {
    const duration = BigInt(getMilliseconds(values.duration));
    const minPrice = BigInt(values.minPrice);

    const sendStartAuctionMessage = () => {
      createAuctionMessage(
        { tokenId, minPrice, duration },
        {
          onSuccess: () => {
            onSuccess();
            alert.success('Auction started');
          },
        },
      );
    };
    const sendAddMarketDataMessage = () => {
      alert.info('NFT approved');
      addMarketDataMessage(
        { tokenId, price: null },
        {
          onSuccess: () => {
            alert.info('Market data added');
            sendStartAuctionMessage();
          },
        },
      );
    };

    if (isMarketOwner) {
      sendStartAuctionMessage();
    } else {
      approveMessage({ tokenId }, { onSuccess: sendAddMarketDataMessage });
    }
  };

  return { buy, offer, bid, settle, startAuction, startSale };
}

export { useMarketplaceActions };
