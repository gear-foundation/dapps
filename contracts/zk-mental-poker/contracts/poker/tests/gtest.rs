#![allow(clippy::type_complexity)]
#![allow(unused)]

use ark_ec::{CurveGroup, PrimeGroup};
use ark_ed_on_bls12_381_bandersnatch::{EdwardsProjective as G, Fq, Fr};
use ark_ff::{BigInteger, PrimeField};
use blake2::{Blake2b512, Digest};
use gtest::WasmProgram;
use hex_literal::hex;

use poker_client::Poker as ClientPoker;
use poker_client::ZkPublicKey;
use poker_client::poker::Poker;
use poker_client::{
    ChaumPedersenProofBytes, Config, PartialDec, PokerCtors, SessionConfig, Stage, Status,
    VerificationVariables,
};

use pts_client::Pts as ClientPts;
use pts_client::PtsCtors;
use pts_client::pts::Pts;

use zk_verification_client::ZkVerificationCtors;

use sails_rs::client::*;
use sails_rs::gtest::{Program, System};
use sails_rs::{ActorId, Encode};

use std::ops::Range;
use std::path::Path;

mod utils_gclient;
use utils_gclient::zk_loader::{ZkLoaderData, ZkSecretKey};
use utils_gclient::{build_player_card_disclosure, init_deck_and_card_map};

const USERS: [u64; 6] = [42, 43, 44, 45, 46, 47];

const BUILTIN_BLS381: ActorId = ActorId::new(hex!(
    "6b6e292c382945e80bf51af2ba7fe9f458dcff81ae6075c46f9095e1bbecdc37"
));

use gbuiltin_bls381::{
    Request, Response,
    ark_bls12_381::{Bls12_381, G1Affine, G1Projective as G1, G2Affine},
    ark_ec::{
        Group, VariableBaseMSM,
        pairing::{MillerLoopOutput, Pairing},
    },
    ark_scale,
    ark_scale::hazmat::ArkScaleProjective,
};

use gstd::prelude::*;

type ArkScale<T> = ark_scale::ArkScale<T, { ark_scale::HOST_CALL }>;
type Gt = <Bls12_381 as Pairing>::TargetField;

type PokerSvc = Service<poker_client::poker::PokerImpl, GtestEnv>;
type PtsSvc = Service<pts_client::pts::PtsImpl, GtestEnv>;

#[test]
fn hash_prefix_agrees() {
    let g = G::generator();
    println!("g = {g:?}");
    let p2 = g + g;
    println!("p2 = {p2:?}");
    let p3 = p2 + g;
    println!("p3 = {p3:?}");
    let result = hash_to_fr(&[g, p2, p3]);
    println!("result = {result:?}");
}

#[tokio::test]
async fn test_check_auto_fold() {
    let (mut env, test_data) = TestEnvironment::setup(TestDataProfile::Basic).await;

    env.register_players(&test_data).await;
    env.start_and_setup_game(&test_data).await;

    env.run_actions(vec![(USERS[2], poker_client::Action::Call)])
        .await;

    let betting = env.poker.betting().await.unwrap();
    println!("betting: {betting:?}");

    for _ in 0..8 {
        env.system().run_next_block();
    }
    for _ in 0..10 {
        env.system().run_next_block();
    }
    for _ in 0..10 {
        env.system().run_next_block();
    }

    env.run_actions(vec![(USERS[0], poker_client::Action::Call)])
        .await;

    let betting = env.poker.betting().await.unwrap();
    println!("betting: {betting:?}");
}

#[tokio::test]
async fn test_basic_poker_workflow() {
    let (mut env, test_data) = TestEnvironment::setup(TestDataProfile::Basic).await;

    env.register_players(&test_data).await;
    env.start_and_setup_game(&test_data).await;

    // preflop
    env.run_actions(vec![
        (USERS[2], poker_client::Action::Call),
        (USERS[3], poker_client::Action::Call),
        (USERS[4], poker_client::Action::Call),
        (USERS[5], poker_client::Action::Call),
        (USERS[0], poker_client::Action::Call),
        (USERS[1], poker_client::Action::Check),
    ])
    .await;

    // reveal 3 cards after preflop
    println!("Decrypt 3 cards after preflop");
    env.reveal_table_cards(&test_data, 0..3).await;

    // flop
    env.run_actions(vec![
        (USERS[0], poker_client::Action::Raise { bet: 50 }),
        (USERS[1], poker_client::Action::Raise { bet: 100 }),
        (USERS[2], poker_client::Action::Call),
        (USERS[3], poker_client::Action::Call),
        (USERS[4], poker_client::Action::Call),
        (USERS[5], poker_client::Action::Call),
        (USERS[0], poker_client::Action::Call),
    ])
    .await;

    // reveal 1 card after flop
    println!("Decrypt 1 card after flop");
    env.reveal_table_cards(&test_data, 3..4).await;

    // turn
    env.run_actions(vec![
        (USERS[0], poker_client::Action::Check),
        (USERS[1], poker_client::Action::Check),
        (USERS[2], poker_client::Action::Check),
        (USERS[3], poker_client::Action::Check),
        (USERS[4], poker_client::Action::Check),
        (USERS[5], poker_client::Action::Check),
    ])
    .await;

    // reveal 1 card after turn
    println!("Decrypt 1 card after turn");
    env.reveal_table_cards(&test_data, 4..5).await;

    env.print_table_cards().await;
    env.reveal_player_cards(&test_data).await;

    env.verify_game_finished().await;

    // check final result
    let result = env.poker.status().await.unwrap();
    let participants = env.poker.participants().await.unwrap();

    println!("participants {participants:?}");

    if let Status::Finished { pots } = result {
        assert_eq!(pots.len(), 1);

        let prize = pots[0].0;
        let winners = pots[0].1.clone();

        participants.iter().for_each(|(id, info)| {
            if winners.contains(id) {
                assert_eq!(
                    info.balance,
                    1000 - 10 - 100 + prize / winners.len() as u128,
                    "Wrong balance!"
                );
            } else {
                assert_eq!(info.balance, 1000 - 10 - 100, "Wrong balance!");
            }
        });
    }
}

#[tokio::test]
async fn gtest_check_null_balance() {
    let (mut env, test_data) = TestEnvironment::setup(TestDataProfile::Basic).await;

    env.register_players(&test_data).await;
    env.start_and_setup_game(&test_data).await;

    // preflop
    env.run_actions(vec![
        (USERS[2], poker_client::Action::Call),
        (USERS[3], poker_client::Action::Call),
        (USERS[4], poker_client::Action::Call),
        (USERS[5], poker_client::Action::Call),
        (USERS[0], poker_client::Action::Call),
        (USERS[1], poker_client::Action::Check),
    ])
    .await;

    println!("Decrypt 3 cards after preflop");
    env.reveal_table_cards(&test_data, 0..3).await;

    // flop
    env.run_actions(vec![
        (USERS[0], poker_client::Action::Raise { bet: 50 }),
        (USERS[1], poker_client::Action::Raise { bet: 100 }),
        (USERS[2], poker_client::Action::Call),
        (USERS[3], poker_client::Action::Call),
        (USERS[4], poker_client::Action::Call),
        (USERS[5], poker_client::Action::Call),
        (USERS[0], poker_client::Action::Call),
    ])
    .await;

    println!("Decrypt 1 card after flop");
    env.reveal_table_cards(&test_data, 3..4).await;

    // turn all-in
    env.run_actions(vec![
        (USERS[0], poker_client::Action::AllIn),
        (USERS[1], poker_client::Action::AllIn),
        (USERS[2], poker_client::Action::AllIn),
        (USERS[3], poker_client::Action::AllIn),
        (USERS[4], poker_client::Action::AllIn),
        (USERS[5], poker_client::Action::AllIn),
    ])
    .await;

    println!("Decrypt 1 card after turn");
    env.reveal_table_cards(&test_data, 4..5).await;

    env.print_table_cards().await;
    env.reveal_player_cards(&test_data).await;

    env.verify_game_finished().await;

    let result = env.poker.status().await.unwrap();
    println!("result {result:?}");
    assert!(matches!(result, Status::Finished { .. }), "Wrong Status!");

    let participants = env.poker.participants().await.unwrap();

    if let Status::Finished { pots } = result {
        let prize = pots[0].0;
        let winners = pots[0].1.clone();
        for winner in winners.iter() {
            participants.iter().for_each(|(id, info)| {
                if winner == id {
                    assert_eq!(
                        info.balance,
                        prize / winners.len() as u128,
                        "Wrong balance!"
                    );
                }
            });
        }
    }

    env.restart_game().await;

    let participants = env.poker.participants().await.unwrap();
    assert_eq!(participants.len(), 1);
}

#[tokio::test]
async fn gtest_one_player_left() {
    let (mut env, test_data) = TestEnvironment::setup(TestDataProfile::Basic).await;

    env.register_players(&test_data).await;
    env.start_and_setup_game(&test_data).await;

    // preflop
    env.run_actions(vec![
        (USERS[2], poker_client::Action::Call),
        (USERS[3], poker_client::Action::Call),
        (USERS[4], poker_client::Action::Call),
        (USERS[5], poker_client::Action::Call),
        (USERS[0], poker_client::Action::Call),
        (USERS[1], poker_client::Action::Check),
    ])
    .await;

    println!("Decrypt 3 cards after preflop");
    env.reveal_table_cards(&test_data, 0..3).await;

    env.check_status(Status::Play { stage: Stage::Flop }).await;

    // flop
    env.run_actions(vec![
        (USERS[0], poker_client::Action::Raise { bet: 50 }),
        (USERS[1], poker_client::Action::Raise { bet: 100 }),
        (USERS[2], poker_client::Action::Call),
        (USERS[3], poker_client::Action::Call),
        (USERS[4], poker_client::Action::Call),
        (USERS[5], poker_client::Action::Call),
        (USERS[0], poker_client::Action::Call),
    ])
    .await;

    env.check_status(Status::Play {
        stage: Stage::WaitingTableCardsAfterFlop,
    })
    .await;

    println!("Decrypt 1 card after flop");
    env.reveal_table_cards(&test_data, 3..4).await;

    env.check_status(Status::Play { stage: Stage::Turn }).await;

    // turn — everyone folds except one
    env.run_actions(vec![
        (USERS[0], poker_client::Action::Fold),
        (USERS[1], poker_client::Action::Fold),
        (USERS[2], poker_client::Action::Fold),
        (USERS[3], poker_client::Action::Fold),
        (USERS[4], poker_client::Action::Fold),
    ])
    .await;

    env.verify_game_finished().await;

    let result = env.poker.status().await.unwrap();
    let participants = env.poker.participants().await.unwrap();

    if let Status::Finished { pots } = result {
        let prize = pots[0].0;
        let winners = pots[0].1.clone();
        for winner in winners.iter() {
            participants.iter().for_each(|(id, info)| {
                if winner == id {
                    assert_eq!(info.balance, 1000 - 10 - 100 + prize, "Wrong balance!");
                } else {
                    assert_eq!(info.balance, 1000 - 10 - 100, "Wrong balance!");
                }
            });
        }
    }

    println!("participants {participants:?}");
}

#[tokio::test]
async fn gtest_delete_player() {
    let (mut env, test_data) = TestEnvironment::setup(TestDataProfile::Basic).await;

    env.register_players(&test_data).await;
    env.delete_player(USERS[1]).await;
    env.register(USERS[1], test_data.pks[1].1.clone()).await;
    env.start_and_setup_game(&test_data).await;
}

#[tokio::test]
async fn gtest_check_cancel_registration_and_turn() {
    let (mut env, test_data) = TestEnvironment::setup(TestDataProfile::Basic).await;

    env.register_players(&test_data).await;
    env.start_and_setup_game(&test_data).await;

    env.run_actions(vec![
        (USERS[2], poker_client::Action::Fold),
        (USERS[3], poker_client::Action::Fold),
        (USERS[4], poker_client::Action::Fold),
        (USERS[5], poker_client::Action::Fold),
        (USERS[0], poker_client::Action::Fold),
    ])
    .await;

    env.verify_game_finished().await;
    env.restart_game().await;
    env.check_status(Status::Registration).await;

    env.start_and_setup_game(&test_data).await;
    env.check_status(Status::Play {
        stage: Stage::PreFlop,
    })
    .await;

    env.run_actions(vec![
        (USERS[2], poker_client::Action::Fold),
        (USERS[3], poker_client::Action::Fold),
        (USERS[4], poker_client::Action::Fold),
        (USERS[5], poker_client::Action::Fold),
        (USERS[0], poker_client::Action::Fold),
    ])
    .await;

    env.verify_game_finished().await;
    env.restart_game().await;

    let active_participants = env.poker.active_participants().await.unwrap();
    println!("active_participants: {active_participants:?}");
    assert_eq!(active_participants.first_index, 0);

    // cancel registration (player 2)
    env.poker
        .cancel_registration(None)
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    let active_participants = env.poker.active_participants().await.unwrap();
    println!("active_participants: {active_participants:?}");
    assert_eq!(active_participants.first_index, 0);
}

#[tokio::test]
async fn gtest_check_cancel_registration_waiting_participants() {
    let (mut env, test_data) = TestEnvironment::setup(TestDataProfile::Basic).await;

    env.register_players(&test_data).await;
    env.start_and_setup_game(&test_data).await;

    let participants = env.participants().await;
    assert_eq!(participants.len(), 6);

    // new player registers while game is running
    let new_player_id: u64 = 48;
    env.system().mint_to(new_player_id, 1_000_000_000_000_000);

    let new_test_data = TestData::load_from_profile(TestDataProfile::SixPlayersNew);
    let new_player_pk = new_test_data.pks[5].1.clone();
    env.register(new_player_id, new_player_pk).await;

    let waiting_participants = env.waiting_participants().await;
    assert_eq!(waiting_participants.len(), 1);

    env.run_actions(vec![
        (USERS[2], poker_client::Action::Fold),
        (USERS[3], poker_client::Action::Fold),
        (USERS[4], poker_client::Action::Fold),
        (USERS[5], poker_client::Action::Fold),
    ])
    .await;

    env.poker
        .cancel_registration(None)
        .with_actor_id(new_player_id.into())
        .await
        .unwrap();

    let waiting_participants = env.waiting_participants().await;
    assert_eq!(waiting_participants.len(), 0);
}

#[tokio::test]
async fn gtest_agg_key_calc() {
    let (mut env, test_data) = TestEnvironment::setup(TestDataProfile::Basic).await;

    env.register_players(&test_data).await;
    env.start_and_setup_game(&test_data).await;

    let participants = env.participants().await;
    assert_eq!(participants.len(), 6);

    let new_player_id: u64 = 48;
    env.system().mint_to(new_player_id, 1_000_000_000_000_000);

    let new_test_data = TestData::load_from_profile(TestDataProfile::SixPlayersNew);
    let new_player_pk = new_test_data.pks[5].1.clone();
    env.register(new_player_id, new_player_pk).await;

    let waiting_participants = env.waiting_participants().await;
    assert_eq!(waiting_participants.len(), 1);

    env.run_actions(vec![
        (USERS[2], poker_client::Action::Fold),
        (USERS[3], poker_client::Action::Fold),
        (USERS[4], poker_client::Action::Fold),
        (USERS[5], poker_client::Action::Fold),
    ])
    .await;

    env.poker
        .cancel_registration(None)
        .with_actor_id(new_player_id.into())
        .await
        .unwrap();

    let waiting_participants = env.waiting_participants().await;
    assert_eq!(waiting_participants.len(), 0);
}

struct TestEnvironment {
    env: GtestEnv,
    pts_id: ActorId,
    poker_id: ActorId,
    pts_program: Actor<pts_client::PtsProgram, GtestEnv>,
    poker_program: Actor<poker_client::PokerProgram, GtestEnv>,
    pts: PtsSvc,
    poker: PokerSvc,
}

struct TestData {
    pks: Vec<(usize, poker_client::ZkPublicKey)>,
    sks: Vec<(usize, ZkSecretKey)>,
    shuffle_proofs: Vec<poker_client::VerificationVariables>,
    encrypted_deck: Vec<poker_client::EncryptedCard>,
    decrypt_proofs: Vec<poker_client::VerificationVariables>,
    table_cards_proofs: Option<
        Vec<(
            poker_client::ZkPublicKey,
            (
                Vec<(poker_client::EncryptedCard, [Vec<u8>; 3])>,
                Vec<poker_client::VerificationVariables>,
            ),
        )>,
    >,
    player_cards: Option<
        Vec<(
            poker_client::ZkPublicKey,
            Vec<utils_gclient::zk_loader::DecryptedCardWithProof>,
        )>,
    >,
}

pub enum TestDataProfile {
    Basic,
    SixPlayers,
    SixPlayersNew,
}

impl TestData {
    pub fn load_from_profile(profile: TestDataProfile) -> Self {
        let prefix = match profile {
            TestDataProfile::Basic => "tests/test_data_gtest/basic",
            TestDataProfile::SixPlayers => "tests/test_data_gtest/6_players_shuffle",
            TestDataProfile::SixPlayersNew => "tests/test_data_gtest/6_players_new_shuffle",
        };

        println!("prefix {prefix:?}");
        let table_path = format!("{prefix}/table_decryptions.json");
        let player_path = format!("{prefix}/player_decryptions.json");

        let table_cards_proofs = if Path::new(&table_path).exists() {
            Some(ZkLoaderData::load_table_cards_proofs(&table_path))
        } else {
            None
        };

        let player_cards = if Path::new(&player_path).exists() {
            Some(ZkLoaderData::load_cards_with_proofs(&player_path))
        } else {
            None
        };

        Self {
            pks: ZkLoaderData::load_player_public_keys(&format!("{prefix}/player_pks.json")),
            sks: ZkLoaderData::load_player_secret_keys(&format!("{prefix}/player_sks.json")),
            shuffle_proofs: ZkLoaderData::load_shuffle_proofs(&format!(
                "{prefix}/shuffle_proofs.json"
            )),
            encrypted_deck: ZkLoaderData::load_encrypted_table_cards(&format!(
                "{prefix}/encrypted_deck.json"
            )),
            decrypt_proofs: ZkLoaderData::load_partial_decrypt_proofs(&format!(
                "{prefix}/partial_decrypt_proofs.json"
            )),
            table_cards_proofs,
            player_cards,
        }
    }
}

impl TestEnvironment {
    fn system(&self) -> &System {
        self.env.system()
    }

    async fn setup(profile: TestDataProfile) -> (Self, TestData) {
        let system = System::new();
        system.init_logger();

        // Mint tokens to users
        for &user_id in &USERS {
            system.mint_to(user_id, 1_000_000_000_000_000);
        }

        // Setup BLS builtin mock
        let builtin_program = Program::mock_with_id(&system, BUILTIN_BLS381, BlsBuiltinMock);
        let init_message_id = builtin_program.send_bytes(USERS[0], b"Doesn't matter");
        let block_run_result = system.run_next_block();
        assert!(block_run_result.succeed.contains(&init_message_id));

        // New env
        let env = GtestEnv::new(system, USERS[0].into());

        // Load test data
        let test_data = TestData::load_from_profile(profile);

        // Deploy PTS
        let pts_program = Self::deploy_pts(&env).await;
        let pts_id = pts_program.id();

        // Deploy Poker (+ zk_verification)
        let poker_program = Self::deploy_poker(&env, pts_id, &test_data.pks[0].1).await;
        let poker_id = poker_program.id();

        // Service handles
        let mut pts = pts_program.pts();
        let poker = poker_program.poker();

        // Add poker program as PTS admin
        pts.add_admin(poker_id).await.unwrap();

        let env = TestEnvironment {
            env,
            pts_id,
            poker_id,
            pts_program,
            poker_program,
            pts,
            poker,
        };

        (env, test_data)
    }

    async fn deploy_pts(env: &GtestEnv) -> Actor<pts_client::PtsProgram, GtestEnv> {
        let pts_code_id = env.system().submit_code(::pts::WASM_BINARY);

        let accural: u128 = 10_000;
        let time_ms_between_balance_receipt: u64 = 10_000;

        env.deploy::<pts_client::PtsProgram>(pts_code_id, b"salt-pts".to_vec())
            .new(accural, time_ms_between_balance_receipt)
            .await
            .unwrap()
    }

    async fn deploy_poker(
        env: &GtestEnv,
        pts_id: ActorId,
        admin_pk: &ZkPublicKey,
    ) -> Actor<poker_client::PokerProgram, GtestEnv> {
        let shuffle_vkey_bytes =
            ZkLoaderData::load_verifying_key("tests/test_data/shuffle_vkey.json");

        // zk verification
        let zk_code_id = env.system().submit_code(::zk_verification::WASM_BINARY);
        let zk_program = env
            .deploy::<zk_verification_client::ZkVerificationProgram>(
                zk_code_id,
                b"salt-zk".to_vec(),
            )
            .new(shuffle_vkey_bytes)
            .await
            .unwrap();

        // poker
        let poker_code_id = env.system().submit_code(::poker::WASM_BINARY);

        env.deploy::<poker_client::PokerProgram>(poker_code_id, b"salt-poker".to_vec())
            .new(
                Config {
                    admin_id: USERS[0].into(),
                    admin_name: "Player_1".to_string(),
                    lobby_name: "Lobby name".to_string(),
                    small_blind: 5,
                    big_blind: 10,
                    starting_bank: 1000,
                    time_per_move_ms: 30_000,
                },
                SessionConfig {
                    gas_to_delete_session: 10_000_000_000,
                    minimum_session_duration_ms: 180_000,
                    ms_per_block: 3_000,
                },
                pts_id,
                admin_pk.clone(),
                None,
                zk_program.id(),
            )
            .await
            .unwrap()
    }

    async fn register_players(&mut self, test_data: &TestData) {
        // 1) PTS: initialize accural for all users
        {
            let pts = &mut self.pts;
            for &user_id in &USERS {
                pts.get_accural()
                    .with_actor_id(user_id.into())
                    .await
                    .unwrap();
            }
        }

        println!("PTS {:?}", self.pts_id);
        // 2) Poker: register players (skip index 0 as admin)
        {
            let poker = &mut self.poker;
            for (i, user) in USERS.iter().enumerate().skip(1) {
                poker
                    .register("Player".to_string(), test_data.pks[i].1.clone(), None)
                    .with_actor_id((*user).into())
                    .await
                    .unwrap();
            }
        }
    }

    async fn start_and_setup_game(&mut self, test_data: &TestData) {
        println!("START GAME");
        self.poker.start_game(None).await.unwrap();
        self.check_status(Status::WaitingShuffleVerification).await;

        println!("SHUFFLE");
        self.poker
            .shuffle_deck(
                test_data.encrypted_deck.clone(),
                test_data.shuffle_proofs.clone(),
            )
            .await
            .unwrap();

        self.check_status(Status::WaitingPartialDecryptionsForPlayersCards)
            .await;

        println!("DECRYPT");

        let partial_decs = get_decs_from_proofs(&test_data.decrypt_proofs);

        let g = G::generator();

        for (i, user) in USERS.iter().enumerate() {
            let pk = deserialize_public_key(&test_data.pks[i].1);
            let sk = test_data.sks[i].1.scalar;

            let mut items = Vec::with_capacity(10);

            for k in 0..10 {
                let c0 = deserialize_bandersnatch_coords(&partial_decs[10 * i + k].0);
                let delta_c0 = deserialize_bandersnatch_coords(&partial_decs[10 * i + k].1);
                let delta_c0_neg = -delta_c0;

                let proof = prove(g, pk, c0, delta_c0_neg, sk);

                items.push(PartialDec {
                    c0: partial_decs[10 * i + k].0.clone(),
                    delta_c0: partial_decs[10 * i + k].1.clone(),
                    proof: proof.to_bytes(),
                });
            }

            self.poker
                .submit_partial_decryptions(items, None)
                .with_actor_id((*user).into())
                .await
                .unwrap();
        }

        self.check_status(Status::Play {
            stage: Stage::PreFlop,
        })
        .await;
    }

    async fn restart_game(&mut self) {
        self.poker.restart_game(None).await.unwrap();
    }

    async fn delete_player(&mut self, id: u64) {
        self.poker.delete_player(id.into(), None).await.unwrap();
    }

    async fn register(&mut self, id: u64, pk: ZkPublicKey) {
        self.pts
            .get_accural()
            .with_actor_id(id.into())
            .await
            .unwrap();

        self.poker
            .register("".to_string(), pk, None)
            .with_actor_id(id.into())
            .await
            .unwrap();
    }

    pub async fn run_actions(&mut self, moves: Vec<(u64, poker_client::Action)>) {
        for (user_id, action) in moves {
            println!("action {action:?}");
            self.poker
                .turn(action, None)
                .with_actor_id(user_id.into())
                .await
                .unwrap();
        }
    }

    pub async fn reveal_table_cards(&mut self, test_data: &TestData, range: Range<usize>) {
        let table_cards_proofs = test_data
            .table_cards_proofs
            .as_ref()
            .expect("No table_cards_proofs for this data profile");

        let g = G::generator();

        for (i, user) in USERS.iter().enumerate() {
            let partial_decs = get_decs_from_proofs(&table_cards_proofs[i].1.1[range.clone()]);

            let pk = deserialize_public_key(&test_data.pks[i].1);
            let sk = test_data.sks[i].1.scalar;

            let mut items = Vec::with_capacity(partial_decs.len());

            for dec in partial_decs {
                let c0 = deserialize_bandersnatch_coords(&dec.0);
                let delta_c0 = deserialize_bandersnatch_coords(&dec.1);
                let delta_c0_neg = -delta_c0;

                let proof = prove(g, pk, c0, delta_c0_neg, sk);

                items.push(PartialDec {
                    c0: dec.0.clone(),
                    delta_c0: dec.1.clone(),
                    proof: proof.to_bytes(),
                });
            }

            self.poker
                .submit_table_partial_decryptions(items, None)
                .with_actor_id((*user).into())
                .await
                .unwrap();
        }
    }

    async fn reveal_player_cards(&mut self, test_data: &TestData) {
        println!("Players reveal their cards..");

        let player_cards = test_data
            .player_cards
            .as_ref()
            .expect("No player_cards for this data profile");

        let (_, card_map) = init_deck_and_card_map();
        let hands = build_player_card_disclosure(player_cards.clone(), &card_map);

        let g = G::generator();

        for i in 0..USERS.len() {
            let proofs = hands[i].1.clone();
            let partial_decs = get_decs_from_proofs(&proofs);

            let pk = deserialize_public_key(&test_data.pks[i].1);
            let sk = test_data.sks[i].1.scalar;

            let mut items = Vec::with_capacity(partial_decs.len());

            for dec in partial_decs {
                let c0 = deserialize_bandersnatch_coords(&dec.0);
                let delta_c0 = deserialize_bandersnatch_coords(&dec.1);
                let delta_c0_neg = -delta_c0;

                let proof = prove(g, pk, c0, delta_c0_neg, sk);

                items.push(PartialDec {
                    c0: dec.0.clone(),
                    delta_c0: dec.1.clone(),
                    proof: proof.to_bytes(),
                });
            }

            self.poker
                .card_disclosure(items, None)
                .with_actor_id(USERS[i].into())
                .await
                .unwrap();
        }
    }

    async fn print_table_cards(&mut self) {
        let table_cards = self.poker.revealed_table_cards().await.unwrap();
        println!("Cards on table {table_cards:?}");
    }

    async fn verify_game_finished(&mut self) -> Status {
        let result = self.poker.status().await.unwrap();
        println!("Final result: {result:?}");

        assert!(
            matches!(result, Status::Finished { .. }),
            "Game should be finished"
        );
        result
    }

    async fn participants(&self) -> Vec<(ActorId, poker_client::Participant)> {
        self.poker.participants().await.unwrap()
    }

    async fn waiting_participants(&self) -> Vec<(ActorId, poker_client::Participant)> {
        self.poker.waiting_participants().await.unwrap()
    }

    async fn check_status(&mut self, expected_status: Status) {
        let result = self.poker.status().await.unwrap();
        assert_eq!(result, expected_status);
    }
}

#[derive(Debug, Clone)]
struct BlsBuiltinMock;

impl WasmProgram for BlsBuiltinMock {
    fn clone_boxed(&self) -> Box<dyn WasmProgram + 'static> {
        Box::new(self.clone())
    }

    fn init(&mut self, _payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
        Ok(Some(vec![]))
    }

    fn handle(&mut self, payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
        let request = Request::decode(&mut payload.as_slice()).expect("Unable to decode payload");

        let result = match request {
            Request::MultiMillerLoop { a, b } => {
                let points_g1 = ArkScale::<Vec<G1Affine>>::decode(&mut a.as_slice())
                    .expect("Unable to decode to Vec<G1Affine>");
                let points_g2 = ArkScale::<Vec<G2Affine>>::decode(&mut b.as_slice())
                    .expect("Unable to decode to Vec<G2Affine>");

                let miller_result: ArkScale<Gt> =
                    Bls12_381::multi_miller_loop(&points_g1.0, &points_g2.0)
                        .0
                        .into();

                Response::MultiMillerLoop(miller_result.encode()).encode()
            }

            Request::FinalExponentiation { f } => {
                let f = ArkScale::<Gt>::decode(&mut f.as_slice()).expect("Unable to decode to Gt");

                let exp_result: ArkScale<Gt> =
                    Bls12_381::final_exponentiation(MillerLoopOutput(f.0))
                        .unwrap()
                        .0
                        .into();

                Response::FinalExponentiation(exp_result.encode()).encode()
            }

            Request::MultiScalarMultiplicationG1 { bases, scalars } => {
                let bases = ArkScale::<Vec<G1Affine>>::decode(&mut bases.as_slice())
                    .expect("Unable to decode to Vec<G1Affine>");
                let scalars =
                    ArkScale::<Vec<<G1 as Group>::ScalarField>>::decode(&mut scalars.as_slice())
                        .expect("Unable to decode to Vec<ScalarField>");

                let result: ArkScaleProjective<G1> = G1::msm(&bases.0, &scalars.0).unwrap().into();
                Response::MultiScalarMultiplicationG1(result.encode()).encode()
            }

            _ => unreachable!(),
        };

        Ok(Some(result))
    }

    fn state(&mut self) -> Result<Vec<u8>, &'static str> {
        Ok(vec![])
    }

    fn debug(&mut self, _data: &str) {}
}

pub fn get_decs_from_proofs(proofs: &[VerificationVariables]) -> Vec<([Vec<u8>; 3], [Vec<u8>; 3])> {
    let mut results = Vec::with_capacity(proofs.len());
    for proof in proofs {
        let c0 = [
            proof.public_input[1].clone(),
            proof.public_input[2].clone(),
            proof.public_input[3].clone(),
        ];
        let dec = [
            proof.public_input[4].clone(),
            proof.public_input[5].clone(),
            proof.public_input[6].clone(),
        ];
        results.push((c0, dec));
    }
    results
}

pub struct ChaumPedersenProof {
    pub a: G,
    pub b: G,
    pub z: Fr,
}

impl ChaumPedersenProof {
    pub fn to_bytes(&self) -> ChaumPedersenProofBytes {
        fn fq_to_bytes(x: &Fq) -> Vec<u8> {
            x.into_bigint().to_bytes_le()
        }

        let a_aff = self.a;
        let b_aff = self.b;

        ChaumPedersenProofBytes {
            a: [
                fq_to_bytes(&a_aff.x),
                fq_to_bytes(&a_aff.y),
                fq_to_bytes(&a_aff.z),
            ],
            b: [
                fq_to_bytes(&b_aff.x),
                fq_to_bytes(&b_aff.y),
                fq_to_bytes(&b_aff.z),
            ],
            z: self.z.into_bigint().to_bytes_le(),
        }
    }
}

fn hash_to_fr(points: &[G]) -> Fr {
    let mut hasher = Blake2b512::new();

    for p in points {
        let affine = p.into_affine();
        let x_bytes = affine.x.into_bigint().to_bytes_le();
        let y_bytes = affine.y.into_bigint().to_bytes_le();
        hasher.update(x_bytes);
        hasher.update(y_bytes);
    }

    let hash_bytes = hasher.finalize();
    Fr::from_le_bytes_mod_order(&hash_bytes[..32])
}

// prove: D = c1^sk and pk = g^sk
pub fn prove(g: G, pk: G, c1: G, d: G, sk: Fr) -> ChaumPedersenProof {
    // Deterministic r: avoids rand dependency in tests
    let r = hash_to_fr(&[g, pk, c1, d]);

    let a = g * r;
    let b = c1 * r;

    let c = hash_to_fr(&[g, pk, c1, d, a, b]);
    let z = r + c * sk;

    ChaumPedersenProof { a, b, z }
}

pub fn verify(g: G, pk: G, c1: G, d: G, proof: &ChaumPedersenProof) -> bool {
    let c = hash_to_fr(&[g, pk, c1, d, proof.a, proof.b]);

    let lhs1 = g * proof.z;
    let rhs1 = proof.a + pk * c;

    let lhs2 = c1 * proof.z;
    let rhs2 = proof.b + d * c;

    lhs1 == rhs1 && lhs2 == rhs2
}

pub fn deserialize_bandersnatch_coords(coords: &[Vec<u8>; 3]) -> G {
    let x = Fq::from_le_bytes_mod_order(&coords[0]);
    let y = Fq::from_le_bytes_mod_order(&coords[1]);
    let z = Fq::from_le_bytes_mod_order(&coords[2]);
    let t = x * y;

    G::new_unchecked(x, y, t, z).into_affine().into()
}

fn deserialize_public_key(pk: &ZkPublicKey) -> G {
    let x = Fq::from_le_bytes_mod_order(&pk.x);
    let y = Fq::from_le_bytes_mod_order(&pk.y);
    let z = Fq::from_le_bytes_mod_order(&pk.z);
    let t = x * y;

    G::new(x, y, t, z)
}
