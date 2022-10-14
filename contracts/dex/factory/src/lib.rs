#![no_std]

use dex_factory_io::*;
use dex_pair_io::*;
use gstd::{exec, msg, prelude::*, prog::ProgramGenerator, ActorId};

#[derive(Debug, Default)]
pub struct Factory {
    // CodeHash to deploy a pair contract from factory.
    pub pair_code_hash: [u8; 32],
    pub owner_id: ActorId,
    // Who gets the fee
    pub fee_to: ActorId,
    pub fee_to_setter: ActorId,
    // (tokenA, tokenB) -> pair_address mapping
    pub pairs: BTreeMap<(ActorId, ActorId), ActorId>,
}

static mut FACTORY: Option<Factory> = None;

impl Factory {
    /// Sets a fee_to address
    /// `fee_to` MUST be a non-zero address
    /// Message source MUST be a fee_to_setter of the contract
    /// Arguments:
    /// * `fee_to` is a new fee_to address
    fn set_fee_to(&mut self, fee_to: ActorId) {
        if self.fee_to_setter != msg::source() {
            panic!("FACTORY: Setting fee_to is forbidden for this address");
        }
        if fee_to == ActorId::zero() {
            panic!("FACTORY: Fee_to can not be a ZERO address");
        }
        self.fee_to = fee_to;

        msg::reply(FactoryEvent::FeeToSet(fee_to), 0)
            .expect("FACTORY: Error during a replying with FactoryEvent::FeeToSet");
    }

    /// Sets a fee_to_setter address
    /// `fee_to_setter` MUST be a non-zero address
    /// Message source MUST be a fee_to_setter of the contract
    /// Arguments:
    /// * `fee_to_setter` is a new fee_to_setter address
    fn set_fee_to_setter(&mut self, fee_to_setter: ActorId) {
        if self.fee_to_setter != msg::source() {
            panic!("FACTORY: Changing fee_to_setter is forbidden for this address");
        }
        if fee_to_setter == ActorId::zero() {
            panic!("FACTORY: Fee_to_setter can not be a ZERO address");
        }
        self.fee_to_setter = fee_to_setter;

        msg::reply(FactoryEvent::FeeToSetterSet(fee_to_setter), 0)
            .expect("FACTORY: Error during a replying with FactoryEvent::FeeToSetterSet");
    }

    /// Creates and deploys a new pair
    /// Both token address MUST be different and non-zero
    /// Also the pair MUST not be created already
    /// Arguments:
    /// * `token_a` is the first token address
    /// * `token_b` is the second token address
    async fn create_pair(&mut self, mut token_a: ActorId, mut token_b: ActorId) {
        (token_a, token_b) = if token_a > token_b {
            (token_b, token_a)
        } else {
            (token_a, token_b)
        };
        if token_a == token_b {
            panic!("FACTORY: Identical token addresses");
        }
        if token_a == ActorId::zero() || token_b == ActorId::zero() {
            panic!("FACTORY: One of your addresses is a ZERO one");
        }
        if self.pairs.contains_key(&(token_a, token_b)) {
            panic!("FACTORY: Such pair already exists.");
        }

        // create program
        let factory_id = &exec::program_id();
        let (_, program_id) = ProgramGenerator::create_program(
            self.pair_code_hash.into(),
            InitPair {
                factory: *factory_id,
                token0: token_a,
                token1: token_b,
            }
            .encode(),
            0,
        )
        .expect("Error in creating pair");

        self.pairs.insert((token_a, token_b), program_id);

        msg::reply(
            FactoryEvent::PairCreated {
                token_a,
                token_b,
                pair_address: program_id,
                pairs_length: self.pairs.len() as u32,
            },
            0,
        )
        .expect("FACTORY: Error during a replying with FactoryEvent::CreatePair");
    }
}

#[no_mangle]
extern "C" fn init() {
    let config: InitFactory = msg::load().expect("Unable to decode InitEscrow");
    if config.fee_to_setter == ActorId::zero() {
        panic!("FACTORY: Fee setter can not be a zero address.");
    }
    let factory = Factory {
        fee_to_setter: config.fee_to_setter,
        owner_id: msg::source(),
        pair_code_hash: config.pair_code_hash,
        ..Default::default()
    };
    unsafe {
        FACTORY = Some(factory);
    }
}

#[gstd::async_main]
async fn main() {
    let action: FactoryAction = msg::load().expect("Unable to decode FactoryAction");
    let factory = unsafe { FACTORY.get_or_insert(Default::default()) };
    match action {
        FactoryAction::SetFeeTo(fee_to) => {
            factory.set_fee_to(fee_to);
        }
        FactoryAction::SetFeeToSetter(fee_to_setter) => {
            factory.set_fee_to_setter(fee_to_setter);
        }
        FactoryAction::CreatePair(token_a, token_b) => {
            factory.create_pair(token_a, token_b).await;
        }
        FactoryAction::FeeTo => {
            msg::reply(FactoryEvent::FeeTo(factory.fee_to), 0)
                .expect("FACTORY: Error during a replying with FactoryEvent::FeeTo");
        }
    }
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let state: FactoryStateQuery = msg::load().expect("Unable to decode FactoryStateQuey");
    let factory = unsafe { FACTORY.get_or_insert(Default::default()) };
    let reply = match state {
        FactoryStateQuery::FeeTo => FactoryStateReply::FeeTo {
            address: factory.fee_to,
        },
        FactoryStateQuery::FeeToSetter => FactoryStateReply::FeeToSetter {
            address: factory.fee_to_setter,
        },
        FactoryStateQuery::PairAddress { token_a, token_b } => {
            let (t1, t2) = if token_a > token_b {
                (token_b, token_a)
            } else {
                (token_a, token_b)
            };
            FactoryStateReply::PairAddress {
                address: factory.pairs.get(&(t1, t2)).cloned().unwrap_or_default(),
            }
        }
        FactoryStateQuery::AllPairsLength => FactoryStateReply::AllPairsLength {
            length: factory.pairs.len() as u32,
        },
        FactoryStateQuery::Owner => FactoryStateReply::Owner {
            address: factory.owner_id,
        },
    };
    gstd::util::to_leak_ptr(reply.encode())
}

gstd::metadata! {
    title: "DEXFactory",
    init:
        input: InitFactory,
    handle:
        input: FactoryAction,
        output: FactoryEvent,
    state:
        input: FactoryStateQuery,
        output: FactoryStateReply,
}
