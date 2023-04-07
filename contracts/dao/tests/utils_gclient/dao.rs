#![allow(unused)]

use super::common;
use dao_io::{DaoAction, DaoEvent, InitDao, Vote};
use gclient::{EventProcessor, GearApi};
use gstd::{prelude::*, ActorId};

const DAO_WASM_PATH: &str = "./target/wasm32-unknown-unknown/debug/dao.opt.wasm";

pub async fn init(
    api: &GearApi,
    ft_program: &ActorId,
    period_duration: u64,
    voting_period_length: u64,
    grace_period_length: u64,
    dilution_bound: u8,
    abort_window: u64,
) -> gclient::Result<ActorId> {
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let init_dao = InitDao {
        approved_token_program_id: *ft_program,
        admin: common::get_current_actor_id(api),
        period_duration,
        voting_period_length,
        grace_period_length,
        dilution_bound,
        abort_window,
    }
    .encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(DAO_WASM_PATH)?,
            init_dao.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(DAO_WASM_PATH)?,
            gclient::now_micros().to_le_bytes(),
            init_dao,
            gas_info.min_limit * 5,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    let program_id: common::Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    Ok(program_id.into())
}

pub async fn add_to_whitelist(
    api: &GearApi,
    program_id: &ActorId,
    account: &ActorId,
    should_fail: bool,
) -> gclient::Result<()> {
    let reply = send_message(api, program_id, DaoAction::AddToWhiteList(*account)).await?;

    if !should_fail {
        let DaoEvent::MemberAddedToWhitelist(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    } else {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn submit_membership_proposal(
    api: &GearApi,
    program_id: &ActorId,
    applicant: &ActorId,
    token_tribute: u128,
    shares_requested: u128,
    quorum: u128,
    details: &str,
    should_fail: bool,
) -> gclient::Result<Option<u128>> {
    let reply = send_message(
        api,
        program_id,
        DaoAction::SubmitMembershipProposal {
            applicant: *applicant,
            token_tribute,
            shares_requested,
            quorum,
            details: details.to_owned(),
        },
    )
    .await?;

    if !should_fail {
        let DaoEvent::SubmitMembershipProposal { proposer: _, applicant: _, proposal_id, token_tribute: _ } = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };

        Ok(Some(proposal_id))
    } else {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };

        Ok(None)
    }
}

pub async fn submit_funding_proposal(
    api: &GearApi,
    program_id: &ActorId,
    applicant: &ActorId,
    amount: u128,
    quorum: u128,
    details: &str,
    should_fail: bool,
) -> gclient::Result<()> {
    let reply = send_message(
        api,
        program_id,
        DaoAction::SubmitFundingProposal {
            applicant: *applicant,
            amount,
            quorum,
            details: details.to_owned(),
        },
    )
    .await?;

    if !should_fail {
        let DaoEvent::SubmitFundingProposal { proposer: _, amount: _, applicant: _, proposal_id: _ } = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    } else {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    }

    Ok(())
}

pub async fn process_proposal(
    api: &GearApi,
    program_id: &ActorId,
    proposal_id: u128,
    should_fail: bool,
) -> gclient::Result<()> {
    let reply = send_message(api, program_id, DaoAction::ProcessProposal(proposal_id)).await?;

    if !should_fail {
        let DaoEvent::ProcessProposal { proposal_id: _, passed: _ } = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    } else {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    }

    Ok(())
}

pub async fn submit_vote(
    api: &GearApi,
    program_id: &ActorId,
    proposal_id: u128,
    vote: Vote,
    should_fail: bool,
) -> gclient::Result<()> {
    let reply = send_message(api, program_id, DaoAction::SubmitVote { proposal_id, vote }).await?;

    if !should_fail {
        let DaoEvent::SubmitVote { account: _, proposal_id: _, vote: _ } = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    } else {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    }

    Ok(())
}

pub async fn rage_quit(
    api: &GearApi,
    program_id: &ActorId,
    shares_amount: u128,
    should_fail: bool,
) -> gclient::Result<()> {
    let reply = send_message(api, program_id, DaoAction::RageQuit(shares_amount)).await?;

    if !should_fail {
        let DaoEvent::RageQuit { member: _, amount: _ } = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    } else {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    }

    Ok(())
}

pub async fn abort(
    api: &GearApi,
    program_id: &ActorId,
    proposal_id: u128,
    should_fail: bool,
) -> gclient::Result<()> {
    let reply = send_message(api, program_id, DaoAction::Abort(proposal_id)).await?;

    if !should_fail {
        let DaoEvent::Abort(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    } else {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    }

    Ok(())
}

pub async fn update_delegate_key(
    api: &GearApi,
    program_id: &ActorId,
    new_delegate: &ActorId,
    should_fail: bool,
) -> gclient::Result<()> {
    let reply = send_message(api, program_id, DaoAction::UpdateDelegateKey(*new_delegate)).await?;

    if !should_fail {
        let DaoEvent::DelegateKeyUpdated { member: _, delegate: _ } = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    } else {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    }

    Ok(())
}

pub async fn set_admin(
    api: &GearApi,
    program_id: &ActorId,
    new_admin: &ActorId,
    should_fail: bool,
) -> gclient::Result<()> {
    let reply = send_message(api, program_id, DaoAction::SetAdmin(*new_admin)).await?;

    if !should_fail {
        let DaoEvent::AdminUpdated(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    } else {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    }

    Ok(())
}

pub async fn cont(
    api: &GearApi,
    program_id: &ActorId,
    tx_id: u64,
    should_fail: bool,
) -> gclient::Result<()> {
    let reply = send_message(api, program_id, DaoAction::Continue(tx_id)).await?;

    if should_fail {
        let DaoEvent::TransactionFailed(_) = DaoEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `DaoEvent` data.") else {
            panic!("Unexpected invalid `DaoEvent`.");
        };
    }

    Ok(())
}

async fn send_message(
    api: &GearApi,
    program_id: &ActorId,
    payload: DaoAction,
) -> gclient::Result<Vec<u8>> {
    let mut listener = api.subscribe().await?;

    let program_id: common::Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id.into(), payload, gas_info.min_limit * 5, 0)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    Ok(reply_data_result.expect("Unexpected invalid reply."))
}
