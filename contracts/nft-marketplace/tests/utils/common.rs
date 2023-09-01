use super::{prelude::*, FungibleToken, Market, NonFungibleToken};
use convert::identity;
use core::fmt::Debug;
use gstd::ActorId;
use gtest::{Log, Program as InnerProgram, RunResult as InnerRunResult, System};
use marker::PhantomData;
use nft_marketplace_io::*;

pub fn initialize_system() -> System {
    let system = System::new();
    system.init_logger();
    system
}

pub fn initialize_programs(
    system: &System,
) -> (FungibleToken<'_>, NonFungibleToken<'_>, Market<'_>) {
    let ft_program = FungibleToken::initialize(system);

    let mut tx_id: u64 = 0;
    let nft_program = NonFungibleToken::initialize(system);
    nft_program.add_minter(tx_id, SELLER);
    tx_id += 1;
    nft_program.mint(tx_id, SELLER);

    let market = Market::initialize(system);
    ft_program.approve(tx_id, BUYER, market.actor_id(), NFT_PRICE);

    tx_id += 1;
    let token_id: TokenId = 0.into();
    nft_program.approve(tx_id, SELLER, market.actor_id(), token_id);
    market
        .add_ft_contract(ADMIN, ft_program.actor_id())
        .succeed(ft_program.actor_id());
    market
        .add_nft_contract(ADMIN, nft_program.actor_id())
        .succeed(nft_program.actor_id());

    (ft_program, nft_program, market)
}

pub fn initialize_programs_without_ft_approve(
    system: &System,
) -> (FungibleToken<'_>, NonFungibleToken<'_>, Market<'_>) {
    let ft_program = FungibleToken::initialize(system);

    let mut tx_id: u64 = 0;
    let nft_program = NonFungibleToken::initialize(system);
    nft_program.add_minter(tx_id, SELLER);
    tx_id += 1;
    nft_program.mint(tx_id, SELLER);

    let market = Market::initialize(system);
    ft_program.approve(tx_id, BUYER, market.actor_id(), NFT_PRICE);

    tx_id += 1;
    let token_id: TokenId = 0.into();
    nft_program.approve(tx_id, SELLER, market.actor_id(), token_id);
    market
        .add_nft_contract(ADMIN, nft_program.actor_id())
        .succeed(nft_program.actor_id());

    (ft_program, nft_program, market)
}

pub trait Program {
    fn inner_program(&self) -> &InnerProgram<'_>;

    fn actor_id(&self) -> ActorId {
        let bytes: [u8; 32] = self.inner_program().id().into();
        bytes.into()
    }
}

pub struct MetaStateReply<T>(pub T);

impl<T: Debug + PartialEq> MetaStateReply<T> {
    #[track_caller]
    pub fn check(self, value: T) {
        assert_eq!(self.0, value);
    }
}

#[must_use]
pub struct RunResult<T, R, E> {
    pub result: InnerRunResult,
    event: fn(T) -> R,
    ghost_data: PhantomData<E>,
}

impl<T, R: Encode, E: Encode> RunResult<T, R, E> {
    pub fn new(result: InnerRunResult, event: fn(T) -> R) -> Self {
        Self {
            result,
            event,
            ghost_data: PhantomData,
        }
    }

    #[track_caller]
    fn assert_contains(self, payload: impl Encode) {
        assert_contains(&self.result, payload);
    }

    #[track_caller]
    pub fn failed(self, error: E) {
        self.assert_contains(Err::<R, E>(error));
    }

    #[track_caller]
    fn common_succeed<V: Encode>(self, value: T, wrap: fn(R) -> V) {
        let event = (self.event)(value);

        self.assert_contains(wrap(event));
    }

    #[track_caller]
    pub fn succeed(self, value: T) {
        self.common_succeed(value, Ok::<R, E>);
    }

    #[track_caller]
    pub fn contains(self, value: T) {
        self.common_succeed(value, identity);
    }
}

#[must_use]
pub struct InitResult<T, E> {
    contract_instance: T,
    pub result: InnerRunResult,
    pub is_active: bool,
    ghost_data: PhantomData<E>,
}

impl<T, E: Encode> InitResult<T, E> {
    pub fn new(contract_instance: T, result: InnerRunResult, is_active: bool) -> Self {
        Self {
            contract_instance,
            result,
            is_active,
            ghost_data: PhantomData,
        }
    }

    fn assert_contains(&self, payload: impl Encode) {
        assert_contains(&self.result, payload);
    }

    #[track_caller]
    pub fn failed(self, error: E) {
        assert!(!self.is_active);
        self.assert_contains(Err::<(), E>(error));
    }

    #[track_caller]
    pub fn succeed(self) -> T {
        assert!(self.is_active);
        self.assert_contains(Ok::<_, E>(()));

        self.contract_instance
    }
}

#[track_caller]
fn assert_contains(result: &InnerRunResult, payload: impl Encode) {
    assert!(result.contains(&Log::builder().payload(payload)));
}
