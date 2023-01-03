use gstd::{prelude::*, ActorId};
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct FTState {
    /// Token name.
    pub name: String,
    /// Token symbol.
    pub symbol: String,
    /// Token's total supply.
    pub total_supply: u128,
    /// Token's decimals.
    pub decimals: u8,
    /// Token holders balances.
    pub balances: HashMap<ActorId, u128>,
    /// Token holders allowance to manipulate token amounts.
    pub allowances: HashMap<ActorId, HashMap<ActorId, u128>>,
}

pub trait FTStateKeeper {
    fn get(&self) -> &FTState;
    fn get_mut(&mut self) -> &mut FTState;
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum FTQuery {
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    BalanceOf { account: ActorId },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum FTQueryReply {
    Name(String),
    Symbol(String),
    Decimals(u8),
    TotalSupply(u128),
    BalanceOf(u128),
}

pub trait FTMetaState: FTStateKeeper {
    fn proc_state(&self, query: FTQuery) -> Option<Vec<u8>> {
        let reply = match query {
            FTQuery::Name => FTQueryReply::Name(self.get().name.clone()),
            FTQuery::Symbol => FTQueryReply::Symbol(self.get().symbol.clone()),
            FTQuery::Decimals => FTQueryReply::Decimals(self.get().decimals),
            FTQuery::TotalSupply => FTQueryReply::TotalSupply(self.get().total_supply),
            FTQuery::BalanceOf { account } => {
                FTQueryReply::BalanceOf(*self.get().balances.get(&account).unwrap_or(&0))
            }
        };
        Some(reply.encode())
    }
}
