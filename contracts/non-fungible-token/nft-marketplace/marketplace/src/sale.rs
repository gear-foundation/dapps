use crate::{
    nft_messages::{nft_payouts, nft_transfer},
    payment::{check_attached_value, transfer_payment},
    Market, MarketEvent,
};
use gstd::{msg, prelude::*, ActorId};
use primitive_types::{H256, U256};

impl Market {
    /// Called when a user wants to buy NFT.
    /// Requirements:
    /// * The NFT must exists and be on sale
    /// * The buyer must have enough balance
    /// * There must be no opened auctions
    /// Arguments:
    /// * `nft_contract_id`: NFT contract address
    /// * `token_id`: the token ID
    pub async fn buy_item(&mut self, nft_contract_id: &ActorId, token_id: U256) {
        let contract_and_token_id =
            format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);
        let item = self
            .items
            .get_mut(&contract_and_token_id)
            .expect("Item does not exist");
        if item.auction.is_some() {
            panic!("There is an opened auction");
        }
        if item.price.is_none() {
            panic!("The item is not on sale");
        }
        check_attached_value(item.ft_contract_id, item.price.unwrap());
        // fee for treasury
        let treasury_fee = item.price.unwrap() * self.treasury_fee / 10_000u128;
        let payouts = nft_payouts(
            nft_contract_id,
            &item.owner_id,
            item.price.unwrap() - treasury_fee,
        )
        .await;
        transfer_payment(
            &msg::source(),
            &self.treasury_id,
            item.ft_contract_id,
            treasury_fee,
        )
        .await;
        for (account, amount) in payouts.iter() {
            transfer_payment(&msg::source(), account, item.ft_contract_id, *amount).await;
        }

        // transfer NFT to buyer
        nft_transfer(nft_contract_id, &msg::source(), token_id).await;

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
        .unwrap();
    }
}
