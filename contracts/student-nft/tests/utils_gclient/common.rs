use super::{student_nft, USERS};
use gclient::GearApi;
use gstd::{prelude::*, ActorId};

pub const HASH_LENGTH: usize = 32;
pub type Hash = [u8; HASH_LENGTH];

pub fn get_current_actor_id(api: &GearApi) -> ActorId {
    ActorId::new(*api.account_id().clone().as_ref())
}

pub async fn get_user_to_actor_id(user: impl AsRef<str>) -> gclient::Result<ActorId> {
    let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH"))
        .await?
        .with(user)?;
    let actor_id = ActorId::new(*api.account_id().clone().as_ref());

    Ok(actor_id)
}

pub async fn fund_users(api: &GearApi) -> gclient::Result<()> {
    let balance = api.total_balance(api.account_id()).await?;
    let amount = balance / (USERS.len() + 1) as u128;

    for user in USERS {
        let user_id = get_user_to_actor_id(user).await?;
        let user_program_id = user_id.encode().as_slice().into();

        api.transfer(user_program_id, amount).await?;
    }

    Ok(())
}

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    fund_users(api).await?;
    student_nft::init(api).await
}
