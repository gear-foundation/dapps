use dex_factory_io::*;
use gstd::{
    errors::Result, exec, msg, prelude::*, prog::ProgramGenerator, ActorId, CodeId, HashMap,
    MessageId,
};

struct Contract {
    pair: CodeId,
    fee_to: ActorId,
    fee_to_setter: ActorId,
    pairs: HashMap<(ActorId, ActorId), ActorId>,
}

static mut STATE: Option<Contract> = None;

impl Contract {
    fn check_fee_to_setter(&self) -> Result<(), Error> {
        if self.fee_to_setter == msg::source() {
            Ok(())
        } else {
            Err(Error::AccessRestricted)
        }
    }

    fn set_fee_to_setter(&mut self, actor: ActorId) -> Result<Event, Error> {
        self.check_fee_to_setter()?;

        if actor.is_zero() {
            return Err(Error::ZeroActorId);
        }

        self.fee_to_setter = actor;

        Ok(Event::FeeToSetterSet(actor))
    }

    fn set_fee_to(&mut self, actor: ActorId) -> Result<Event, Error> {
        self.check_fee_to_setter()?;

        self.fee_to = actor;

        Ok(Event::FeeToSet(actor))
    }

    async fn create_pair(&mut self, token_a: ActorId, token_b: ActorId) -> Result<Event, Error> {
        if token_a == token_b {
            return Err(Error::IdenticalTokens);
        }

        if token_a.is_zero() || token_b.is_zero() {
            return Err(Error::ZeroActorId);
        }

        let token_pair = if token_b > token_a {
            (token_b, token_a)
        } else {
            (token_a, token_b)
        };

        if self.pairs.contains_key(&token_pair) {
            return Err(Error::PairExist);
        }

        let (pair_actor, result): (_, Result<(), dex_pair_io::Error>) =
            ProgramGenerator::create_program_for_reply_as(
                self.pair,
                dex_pair_io::Initialize {
                    pair: token_pair,
                    factory: exec::program_id(),
                }
                .encode(),
                0,
                0,
            )?
            .await?;

        result?;

        self.pairs.insert(token_pair, pair_actor);

        Ok(Event::PairCreated {
            token_pair,
            pair_actor,
            pair_number: self.pairs.len().try_into().unwrap(),
        })
    }
}

#[no_mangle]
extern "C" fn init() {
    let result = process_init();
    let is_err = result.is_err();

    reply(result).expect("failed to encode or reply from `init()`");

    if is_err {
        exec::exit(ActorId::zero());
    }
}

fn process_init() -> Result<(), Error> {
    let Initialize {
        fee_to,
        pair,
        fee_to_setter,
    } = msg::load()?;

    unsafe {
        STATE = Some(Contract {
            pair,
            fee_to,
            fee_to_setter,
            pairs: HashMap::new(),
        });
    };

    Ok(())
}

#[gstd::async_main]
async fn main() {
    reply(process_handle().await).expect("failed to encode or reply `handle()`");
}

async fn process_handle() -> Result<Event, Error> {
    let action: Action = msg::load()?;
    let contract = state_mut();

    match action {
        Action::FeeToSetter(actor) => contract.set_fee_to_setter(actor),
        Action::FeeTo(actor) => contract.set_fee_to(actor),
        Action::CreatePair(token_a, token_b) => contract.create_pair(token_a, token_b).await,
        Action::GetFeeTo => Ok(Event::FeeToSet(contract.fee_to)),
    }
}

fn state_mut() -> &'static mut Contract {
    unsafe { STATE.as_mut().expect("state isn't initialized") }
}

#[no_mangle]
extern "C" fn state() {
    let Contract {
        pair,
        fee_to,
        fee_to_setter: admin,
        pairs,
    } = state_mut();

    reply(State {
        pair: *pair,
        fee_to_setter: *admin,
        fee_to: *fee_to,
        pairs: pairs.into_iter().map(|(k, v)| (*k, *v)).collect(),
    })
    .expect("failed to encode or reply from `state()`");
}

fn reply(payload: impl Encode) -> Result<MessageId> {
    msg::reply(payload, 0)
}
