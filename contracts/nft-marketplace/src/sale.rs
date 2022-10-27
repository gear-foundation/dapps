use crate::{nft_messages::*, payment::*, Market, MarketEvent, BASE_PERCENT};
use gstd::{msg, prelude::*, ActorId};
use primitive_types::{H256, U256};

impl Market {
    pub async fn buy_item(&mut self, nft_contract_id: &ActorId, token_id: U256) {
        let contract_and_token_id =
            format!("{}{token_id}", H256::from_slice(nft_contract_id.as_ref()));
        let item = self
            .items
            .get_mut(&contract_and_token_id)
            .expect("Item does not exist");
        if item.auction.is_some() {
            panic!("There is an opened auction");
        }
        let price = item.price.expect("The item is not on sale");

        check_attached_value(item.ft_contract_id, price);
        // fee for treasury
        let treasury_fee = price * (self.treasury_fee * BASE_PERCENT) as u128 / 10_000u128;

        transfer_payment(
            &msg::source(),
            &self.treasury_id,
            item.ft_contract_id,
            treasury_fee,
        )
        .await;

        // transfer NFT and pay royalties
        let payouts = nft_transfer(
            nft_contract_id,
            &msg::source(),
            token_id,
            price - treasury_fee,
        )
        .await;
        for (account, amount) in payouts.iter() {
            transfer_payment(&msg::source(), account, item.ft_contract_id, *amount).await;
        }

        item.owner_id = msg::source();
        item.price = None;

        msg::reply(
            MarketEvent::ItemSold {
                owner: msg::source(),
                nft_contract_id: *nft_contract_id,
                token_id,
            },
            0,
        )
        .expect("Error in reply [MarketEvent::ItemSold]");
    }
}
