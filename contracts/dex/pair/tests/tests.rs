use utils::{prelude::*, FungibleToken};

mod utils;

const USERS: &[u64] = &[5, 6, 7];
const INIT_AMOUNT: u128 = 1000000;
const INIT_LIQ: u128 = INIT_AMOUNT / 2;
const CLEAN_INIT_LIQ: u128 = INIT_AMOUNT / 2 - 1000;

#[test]
fn swaps_and_fee() {
    const SWAP_AMOUNT: u128 = 100000;

    let system = utils::initialize_system();

    let mut fungible_token_b = FungibleToken::initialize(&system);
    let mut fungible_token_a = FungibleToken::initialize(&system);

    // Initialization of the contracts

    let mut factory = Factory::initialize(&system, USERS[2], 0, 3).succeed();
    let actor_pair = (fungible_token_a.actor_id(), fungible_token_b.actor_id());
    let pair_actor = factory.create_pair(actor_pair).succeed((actor_pair, 1));
    let mut pair = Pair(system.get_program(pair_actor));

    // Checking the initialization results

    factory.state().pair(actor_pair).eq(pair.actor_id());
    pair.state().factory().eq(factory.actor_id());
    pair.state().token().eq(actor_pair);

    fungible_token_a.mint(USERS[0], INIT_AMOUNT);
    fungible_token_b.mint(USERS[0], INIT_AMOUNT);
    fungible_token_a.approve(USERS[0], pair.actor_id(), INIT_LIQ);
    fungible_token_b.approve(USERS[0], pair.actor_id(), INIT_LIQ);

    // Adding liquidity

    pair.add_liquidity(USERS[0], (INIT_LIQ, INIT_LIQ), (0, 0), USERS[0])
        .succeed((USERS[0], (INIT_LIQ, INIT_LIQ), CLEAN_INIT_LIQ));

    // Checking the adding results

    pair.state().balance_of(USERS[0]).eq(CLEAN_INIT_LIQ);
    pair.state().reserve().eq((INIT_LIQ, INIT_LIQ));

    // SwapExactTokensForTokens AForB

    let mut swap_kind = SwapKind::AForB;
    let mut pair_reserve = (INIT_LIQ, INIT_LIQ);
    let mut user_balance = (INIT_AMOUNT - INIT_LIQ, INIT_AMOUNT - INIT_LIQ);
    let mut cumulative_price = (U256::zero(), U256::zero());
    let mut out_amount = pair
        .state()
        .calculate_out_amount(swap_kind, SWAP_AMOUNT)
        .0
        .unwrap();

    fungible_token_a.approve(USERS[0], pair.actor_id(), SWAP_AMOUNT);
    system.spend_blocks(SPENT_BLOCKS);
    pair.swap_exact_tokens_for_tokens(USERS[0], (SWAP_AMOUNT, 0), USERS[0], swap_kind)
        .succeed((USERS[0], (SWAP_AMOUNT, out_amount), USERS[0], swap_kind));

    cumulative_price.0 += utils::calculate_cp(pair_reserve);
    cumulative_price.1 += utils::calculate_cp((pair_reserve.1, pair_reserve.0));
    pair_reserve.0 += SWAP_AMOUNT;
    pair_reserve.1 -= out_amount;
    user_balance.0 -= SWAP_AMOUNT;
    user_balance.1 += out_amount;

    fungible_token_a
        .balance(pair.actor_id())
        .contains(pair_reserve.0);
    fungible_token_b
        .balance(pair.actor_id())
        .contains(pair_reserve.1);
    fungible_token_a.balance(USERS[0]).contains(user_balance.0);
    fungible_token_b.balance(USERS[0]).contains(user_balance.1);
    pair.state().price().eq(cumulative_price);
    pair.state().reserve().eq(pair_reserve);

    // SwapTokensForExactTokens AForB

    let mut in_amount = pair
        .state()
        .calculate_in_amount(swap_kind, SWAP_AMOUNT)
        .0
        .unwrap();

    fungible_token_a.approve(USERS[0], pair.actor_id(), in_amount);
    system.spend_blocks(SPENT_BLOCKS);
    pair.swap_tokens_for_exact_tokens(USERS[0], (SWAP_AMOUNT, 99999999), USERS[0], swap_kind)
        .succeed((USERS[0], (in_amount, SWAP_AMOUNT), USERS[0], swap_kind));

    cumulative_price.0 += utils::calculate_cp(pair_reserve);
    cumulative_price.1 += utils::calculate_cp((pair_reserve.1, pair_reserve.0));
    pair_reserve.0 += in_amount;
    pair_reserve.1 -= SWAP_AMOUNT;
    user_balance.0 -= in_amount;
    user_balance.1 += SWAP_AMOUNT;

    fungible_token_a
        .balance(pair.actor_id())
        .contains(pair_reserve.0);
    fungible_token_b
        .balance(pair.actor_id())
        .contains(pair_reserve.1);
    fungible_token_a.balance(USERS[0]).contains(user_balance.0);
    fungible_token_b.balance(USERS[0]).contains(user_balance.1);
    pair.state().price().eq(cumulative_price);
    pair.state().reserve().eq(pair_reserve);

    // SwapExactTokensForTokens BForA

    swap_kind = SwapKind::BForA;
    out_amount = pair
        .state()
        .calculate_out_amount(swap_kind, SWAP_AMOUNT)
        .0
        .unwrap();

    fungible_token_b.approve(USERS[0], pair.actor_id(), SWAP_AMOUNT);
    system.spend_blocks(SPENT_BLOCKS);
    pair.swap_exact_tokens_for_tokens(USERS[0], (SWAP_AMOUNT, 0), USERS[0], swap_kind)
        .succeed((USERS[0], (SWAP_AMOUNT, out_amount), USERS[0], swap_kind));

    cumulative_price.0 += utils::calculate_cp(pair_reserve);
    cumulative_price.1 += utils::calculate_cp((pair_reserve.1, pair_reserve.0));
    pair_reserve.1 += SWAP_AMOUNT;
    pair_reserve.0 -= out_amount;
    user_balance.1 -= SWAP_AMOUNT;
    user_balance.0 += out_amount;

    fungible_token_a
        .balance(pair.actor_id())
        .contains(pair_reserve.0);
    fungible_token_b
        .balance(pair.actor_id())
        .contains(pair_reserve.1);
    fungible_token_a.balance(USERS[0]).contains(user_balance.0);
    fungible_token_b.balance(USERS[0]).contains(user_balance.1);
    pair.state().price().eq(cumulative_price);
    pair.state().reserve().eq(pair_reserve);

    // SwapTokensForExactTokens BForA

    in_amount = pair
        .state()
        .calculate_in_amount(swap_kind, SWAP_AMOUNT)
        .0
        .unwrap();

    fungible_token_b.approve(USERS[0], pair.actor_id(), in_amount);
    system.spend_blocks(SPENT_BLOCKS);
    pair.swap_tokens_for_exact_tokens(USERS[0], (SWAP_AMOUNT, 9999999999), USERS[0], swap_kind)
        .succeed((USERS[0], (in_amount, SWAP_AMOUNT), USERS[0], swap_kind));

    cumulative_price.0 += utils::calculate_cp(pair_reserve);
    cumulative_price.1 += utils::calculate_cp((pair_reserve.1, pair_reserve.0));
    pair_reserve.1 += in_amount;
    pair_reserve.0 -= SWAP_AMOUNT;
    user_balance.1 -= in_amount;
    user_balance.0 += SWAP_AMOUNT;

    fungible_token_a
        .balance(pair.actor_id())
        .contains(pair_reserve.0);
    fungible_token_b
        .balance(pair.actor_id())
        .contains(pair_reserve.1);
    fungible_token_a.balance(USERS[0]).contains(user_balance.0);
    fungible_token_b.balance(USERS[0]).contains(user_balance.1);
    pair.state().price().eq(cumulative_price);
    pair.state().reserve().eq(pair_reserve);

    // Liqtoken transfer and fee collection

    pair.transfer(USERS[0], CLEAN_INIT_LIQ, USERS[1])
        .succeed((USERS[0], USERS[1], CLEAN_INIT_LIQ));
    pair.state().balance_of(USERS[0]).eq(0);
    pair.state().balance_of(USERS[1]).eq(CLEAN_INIT_LIQ);

    let U256PairTuple(reserve) = pair_reserve.into();
    let root_k = (reserve.0 * reserve.1).integer_sqrt().low_u128();
    let root_k_last = INIT_LIQ;
    let fee = (INIT_LIQ * (root_k - root_k_last)) / (root_k * 5 + root_k_last);
    let mut total_supply_with_fee = INIT_LIQ + fee;
    let returned_amount = (
        CLEAN_INIT_LIQ * pair_reserve.0 / total_supply_with_fee,
        CLEAN_INIT_LIQ * pair_reserve.1 / total_supply_with_fee,
    );

    pair.remove_liquidity(USERS[1], CLEAN_INIT_LIQ, (0, 0), USERS[1])
        .succeed((USERS[1], returned_amount, USERS[1]));

    pair_reserve.0 -= returned_amount.0;
    pair_reserve.1 -= returned_amount.1;
    total_supply_with_fee -= CLEAN_INIT_LIQ;

    pair.state().balance_of(USERS[1]).eq(0);
    pair.state().reserve().eq(pair_reserve);
    fungible_token_a
        .balance(USERS[1])
        .contains(returned_amount.0);
    fungible_token_b
        .balance(USERS[1])
        .contains(returned_amount.1);

    pair.remove_liquidity(USERS[2], fee, (0, 0), USERS[2])
        .succeed((
            USERS[2],
            (
                fee * pair_reserve.0 / total_supply_with_fee,
                fee * pair_reserve.1 / total_supply_with_fee,
            ),
            USERS[2],
        ));
}

#[test]
fn swap_errors() {
    let system = utils::initialize_system();

    let mut fungible_token_b = FungibleToken::initialize(&system);
    let mut fungible_token_a = FungibleToken::initialize(&system);

    let mut factory = Factory::initialize(&system, 0, 0, 3).succeed();
    let actor_pair = (fungible_token_a.actor_id(), fungible_token_b.actor_id());
    let pair_actor = factory.create_pair(actor_pair).succeed((actor_pair, 1));
    let mut pair = Pair(system.get_program(pair_actor));

    pair.swap_exact_tokens_for_tokens_with_deadline(USERS[0], (0, 0), USERS[0], SwapKind::AForB, 0)
        .failed(Error::DeadlineExceeded);
    pair.swap_exact_tokens_for_tokens(
        USERS[0],
        (0, 0),
        fungible_token_a.actor_id(),
        SwapKind::AForB,
    )
    .failed(Error::InvalidRecipient);
    pair.swap_exact_tokens_for_tokens(
        USERS[0],
        (0, 0),
        fungible_token_b.actor_id(),
        SwapKind::AForB,
    )
    .failed(Error::InvalidRecipient);

    pair.swap_tokens_for_exact_tokens_with_deadline(USERS[0], (0, 0), USERS[0], SwapKind::AForB, 0)
        .failed(Error::DeadlineExceeded);
    pair.swap_tokens_for_exact_tokens(
        USERS[0],
        (0, 0),
        fungible_token_a.actor_id(),
        SwapKind::AForB,
    )
    .failed(Error::InvalidRecipient);
    pair.swap_tokens_for_exact_tokens(
        USERS[0],
        (0, 0),
        fungible_token_b.actor_id(),
        SwapKind::AForB,
    )
    .failed(Error::InvalidRecipient);

    fungible_token_a.mint(pair.actor_id(), INIT_LIQ);
    fungible_token_b.mint(pair.actor_id(), INIT_LIQ);
    pair.sync().succeed((INIT_LIQ, INIT_LIQ));

    pair.swap_exact_tokens_for_tokens(USERS[0], (1, 1), USERS[0], SwapKind::AForB)
        .failed(Error::InsufficientLatterAmount);
    pair.swap_tokens_for_exact_tokens(USERS[0], (1, 1), USERS[0], SwapKind::AForB)
        .failed(Error::InsufficientFormerAmount);
}

#[test]
fn factory() {
    let system = utils::initialize_system();
    let mut factory = Factory::initialize(&system, 0, USERS[0], 1).succeed();

    factory.state().fee_to_setter().eq(USERS[0].into());
    factory.state().fee_to().eq(ActorId::zero());

    // FeeToSetter

    factory
        .fee_to_setter(USERS[0], ActorId::zero())
        .failed(dex_factory_io::Error::ZeroActorId);
    factory
        .fee_to_setter(FOREIGN_USER, ActorId::zero())
        .failed(dex_factory_io::Error::AccessRestricted);

    factory.fee_to_setter(USERS[0], USERS[1]).succeed(USERS[1]);
    factory.state().fee_to_setter().eq(USERS[1].into());

    // FeeTo

    factory
        .fee_to(USERS[0], ActorId::zero())
        .failed(dex_factory_io::Error::AccessRestricted);
    factory
        .fee_to(USERS[1], ActorId::zero())
        .succeed(ActorId::zero());
    factory.state().fee_to().eq(ActorId::zero());

    factory.fee_to(USERS[1], USERS[1]).succeed(USERS[1].into());
    factory.state().fee_to().eq(USERS[1].into());

    // CreatePair

    let fungible_token_b = FungibleToken::initialize(&system);
    let fungible_token_a = FungibleToken::initialize(&system);
    let actor_pair = (fungible_token_a.actor_id(), fungible_token_b.actor_id());

    factory.state().all_pairs().eq(vec![]);
    factory.state().all_pairs_length().eq(0);
    factory.state().pair(actor_pair).eq(ActorId::zero());

    let pair_actor = factory
        .create_pair(actor_pair)
        .succeed((actor_pair, 1))
        .into();

    factory
        .state()
        .all_pairs()
        .eq(vec![(actor_pair, pair_actor)]);
    factory.state().all_pairs_length().eq(1);
    factory.state().pair(actor_pair).eq(pair_actor);

    factory
        .create_pair(actor_pair)
        .failed(dex_factory_io::Error::PairExist);
}

#[test]
fn add_liquidity() {
    let system = utils::initialize_system();

    let mut fungible_token_b = FungibleToken::initialize(&system);
    let mut fungible_token_a = FungibleToken::initialize(&system);

    let mut factory = Factory::initialize(&system, 0, 0, 3).succeed();
    let actor_pair = (fungible_token_a.actor_id(), fungible_token_b.actor_id());
    let pair_actor = factory.create_pair(actor_pair).succeed((actor_pair, 1));
    let mut pair = Pair(system.get_program(pair_actor));

    fungible_token_b.mint(pair.actor_id(), u128::MAX);
    pair.sync().succeed((0, u128::MAX));

    pair.add_liquidity_with_deadline(USERS[0], (0, 0), (0, 0), USERS[0], 0)
        .failed(Error::DeadlineExceeded);
    // A error because `reserve.0` == 0.
    pair.add_liquidity(USERS[0], (u128::MAX, 0), (0, 0), USERS[0])
        .failed(Error::InsufficientLiquidity);

    fungible_token_a.mint(pair.actor_id(), 2);
    pair.sync().succeed((2, u128::MAX));

    // amount      * reserve.1   / reserve.0 > u128::MAX
    // `u128::MAX` * `u128::MAX` / 1         > u128::MAX
    pair.add_liquidity(USERS[0], (u128::MAX, 0), (0, 0), USERS[0])
        .failed(Error::Overflow);
    // amount.0 == 0
    pair.add_liquidity(USERS[0], (0, u128::MAX), (0, 0), USERS[0])
        .failed(Error::InsufficientAmount);
    // amount.1 == 0
    pair.add_liquidity(USERS[0], (1, 0), (0, 0), USERS[0])
        .failed(Error::InsufficientAmount);
    // optimal_amount_b < min_amount.1
    pair.add_liquidity(USERS[0], (1, u128::MAX), (0, u128::MAX), USERS[0])
        .failed(Error::InsufficientLatterAmount);
    // optimal_amount_a < min_amount.0
    pair.add_liquidity(USERS[0], (1, 1), (1, 0), USERS[0])
        .failed(Error::InsufficientFormerAmount);
    // `reserve.1 == u128::MAX`, so `reserve.1 + 1 > u128::MAX`
    pair.add_liquidity(USERS[0], (1, 1), (0, 0), USERS[0])
        .failed(Error::Overflow);
}

#[test]
fn add_liquidity_2() {
    let system = utils::initialize_system();

    let fungible_token_b = FungibleToken::initialize(&system);
    let mut fungible_token_a = FungibleToken::initialize(&system);

    let mut factory = Factory::initialize(&system, 0, 0, 3).succeed();
    let actor_pair = (fungible_token_a.actor_id(), fungible_token_b.actor_id());
    let pair_actor = factory.create_pair(actor_pair).succeed((actor_pair, 1));
    let mut pair = Pair(system.get_program(pair_actor));

    // 0 < MINIMUM_LIQUIDITY
    pair.add_liquidity(USERS[0], (0, 0), (0, 0), USERS[0])
        .failed(Error::InsufficientAmount);
    // Added liquidity must be > `MINIMUM_LIQUIDITY`.
    pair.add_liquidity(
        USERS[0],
        (MINIMUM_LIQUIDITY.into(), MINIMUM_LIQUIDITY.into()),
        (0, 0),
        USERS[0],
    )
    .failed(Error::InsufficientLiquidity);

    // Transfer fails

    let min_liq_and_one = MINIMUM_LIQUIDITY as u128 + 1;
    pair.add_liquidity(
        USERS[0],
        (min_liq_and_one, min_liq_and_one),
        (0, 0),
        USERS[0],
    )
    .failed(Error::TransferFailed);
    // Refund check
    fungible_token_a.mint(USERS[0], min_liq_and_one);
    fungible_token_a.approve(USERS[0], pair.actor_id(), min_liq_and_one);
    pair.add_liquidity(
        USERS[0],
        (min_liq_and_one, min_liq_and_one),
        (0, 0),
        USERS[0],
    )
    .failed(Error::TransferFailed);
}

#[test]
fn remove_liquidity() {
    let system = utils::initialize_system();

    let mut fungible_token_b = FungibleToken::initialize(&system);
    let mut fungible_token_a = FungibleToken::initialize(&system);

    let mut factory = Factory::initialize(&system, 0, 0, 3).succeed();
    let actor_pair = (fungible_token_a.actor_id(), fungible_token_b.actor_id());
    let pair_actor = factory.create_pair(actor_pair).succeed((actor_pair, 1));
    let mut pair = Pair(system.get_program(pair_actor));

    pair.remove_liquidity_with_deadline(USERS[0], 0, (0, 0), USERS[0], 0)
        .failed(Error::DeadlineExceeded);
    pair.remove_liquidity(USERS[0], 1, (0, 0), USERS[0])
        .failed(Error::InsufficientLiquidity);

    let min_liq_and_one = MINIMUM_LIQUIDITY as u128 + 1;
    fungible_token_a.mint(USERS[0], min_liq_and_one);
    fungible_token_b.mint(USERS[0], min_liq_and_one);
    fungible_token_a.approve(USERS[0], pair.actor_id(), min_liq_and_one);
    fungible_token_b.approve(USERS[0], pair.actor_id(), min_liq_and_one);
    pair.add_liquidity(
        USERS[0],
        (min_liq_and_one, min_liq_and_one),
        (0, 0),
        USERS[0],
    )
    .succeed((USERS[0], (min_liq_and_one, min_liq_and_one), 1));

    pair.remove_liquidity(USERS[0], 1, (2, 0), USERS[0])
        .failed(Error::InsufficientFormerAmount);
    pair.remove_liquidity(USERS[0], 1, (0, 2), USERS[0])
        .failed(Error::InsufficientLatterAmount);
}
