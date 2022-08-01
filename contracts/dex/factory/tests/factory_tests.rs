use dex_factory_io::*;
use gstd::{prelude::*, ActorId};
use gtest::System;
mod utils;

pub const USER: u64 = 10;
pub const FEE_SETTER: u64 = 11;
pub const NEW_FEE_SETTER: u64 = 12;
pub const NEW_FEE_TO: u64 = 13;
pub const TOKEN_A: u64 = 101;
pub const TOKEN_B: u64 = 102;

#[test]
fn fee_to() {
    let sys = System::new();
    utils::init_factory(&sys);
    let factory = sys.get_program(1);
    utils::set_fee_to(&factory, FEE_SETTER, ActorId::from(NEW_FEE_TO));
    let res = utils::fee_to(&factory, FEE_SETTER);
    let message = FactoryEvent::FeeTo(ActorId::from(NEW_FEE_TO)).encode();
    assert!(res.contains(&(FEE_SETTER, message)));
}

#[test]
fn set_fee_to() {
    let sys = System::new();
    utils::init_factory(&sys);
    let factory = sys.get_program(1);
    // should be sent by a fee_setter
    let res = utils::set_fee_to(&factory, FEE_SETTER, ActorId::from(NEW_FEE_TO));
    let message = FactoryEvent::FeeToSet(ActorId::from(NEW_FEE_TO)).encode();
    assert!(res.contains(&(FEE_SETTER, message)));

    // check if new fee_to is in state
    utils::check_fee_to(&factory, ActorId::from(NEW_FEE_TO));
}

#[test]
fn set_fee_to_failures() {
    let sys = System::new();
    utils::init_factory(&sys);
    let factory = sys.get_program(1);
    // MUST fail since the USER is not a fee setter
    let res = utils::set_fee_to(&factory, USER, ActorId::from(NEW_FEE_TO));
    assert!(res.main_failed());
    // MUST fail since the NEW_FEE_TO a ActorId::zero() address
    let res = utils::set_fee_to(&factory, USER, ActorId::zero());
    assert!(res.main_failed());
}

#[test]
fn set_fee_to_setter() {
    let sys = System::new();
    utils::init_factory(&sys);
    let factory = sys.get_program(1);
    let res = utils::set_fee_to_setter(&factory, FEE_SETTER, ActorId::from(NEW_FEE_SETTER));
    let message = FactoryEvent::FeeToSetterSet(ActorId::from(NEW_FEE_SETTER)).encode();
    assert!(res.contains(&(FEE_SETTER, message)));
    // check if new fee_to_setter is in state
    utils::check_fee_to_setter(&factory, ActorId::from(NEW_FEE_SETTER));
}

#[test]
fn set_fee_to_setter_failures() {
    let sys = System::new();
    utils::init_factory(&sys);
    let factory = sys.get_program(1);
    // MUST fail since the USER is not a fee setter
    let res = utils::set_fee_to_setter(&factory, USER, ActorId::from(NEW_FEE_SETTER));
    assert!(res.main_failed());
    // MUST fail since the NEW_FEE_TO_SETTER a ActorId::zero() address
    let res = utils::set_fee_to_setter(&factory, FEE_SETTER, ActorId::zero());
    assert!(res.main_failed());
}

#[test]
fn create_pair() {
    let sys = System::new();
    utils::init_factory(&sys);
    let factory = sys.get_program(1);
    let token_a = ActorId::from(TOKEN_A);
    let token_b = ActorId::from(TOKEN_B);
    // MUST fail since token_a and token_b share the same address
    let res = utils::create_pair(&factory, USER, token_a, token_b);
    // There is no way to fully check against PairCreated
    // because of the pair_address being random
    // we should check for logs being non empty and not failed
    assert!(!res.main_failed());
    assert!(!res.log().is_empty());

    // check if the all pair length is equal to 1
    utils::check_pair_len(&factory, 1);
}

#[test]
fn create_pair_failures() {
    let sys = System::new();
    utils::init_factory(&sys);
    let factory = sys.get_program(1);
    let token_a = ActorId::from(TOKEN_A);
    let token_b = ActorId::from(TOKEN_B);
    // MUST fail since token_a and token_b share the same address
    utils::create_pair(&factory, USER, token_a, token_a);
    // MUST fail since token_a is a ActorId::zero() address
    utils::create_pair(&factory, USER, ActorId::zero(), token_a);
    // MUST fail since the pair already exists
    utils::create_pair(&factory, USER, token_a, token_b);
    utils::create_pair(&factory, USER, token_a, token_b);
}
