use super::vara_man;
use blake2_rfc::blake2b;
use gclient::{Error as GclientError, GearApi};
use gstd::{prelude::*, ActorId};
use vara_man_io::Config;

pub const HASH_LENGTH: usize = 32;
pub type Hash = [u8; HASH_LENGTH];

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    let vara_man = vara_man::init(api).await?;

    // Fund users
    let destination = get_user_to_actor_id("//Peter")
        .await?
        .encode()
        .as_slice()
        .into();
    api.transfer(destination, api.total_balance(api.account_id()).await? / 2)
        .await?;

    Ok(vara_man)
}

pub async fn init_with_config(api: &GearApi, config: Config) -> gclient::Result<ActorId> {
    let vara_man = vara_man::init_with_config(api, config).await?;

    Ok(vara_man)
}

pub fn get_current_actor_id(api: &GearApi) -> ActorId {
    ActorId::new(*api.account_id().clone().as_ref())
}

pub async fn get_user_to_actor_id(user: impl AsRef<str>) -> gclient::Result<ActorId> {
    let api = GearApi::dev().await?.with(user)?;
    let actor_id = ActorId::new(*api.account_id().clone().as_ref());

    Ok(actor_id)
}

pub async fn upload_with_code_hash(
    api: &GearApi,
    wasm_path: impl AsRef<str>,
) -> gclient::Result<Hash> {
    let mut code_hash: Hash = Default::default();
    let wasm_code = gclient::code_from_os(wasm_path.as_ref())?;

    code_hash[..].copy_from_slice(blake2b::blake2b(HASH_LENGTH, &[], &wasm_code).as_bytes());

    match api.upload_code(wasm_code).await {
        // Catch re-upload
        Err(GclientError::ProgramAlreadyExists(_)) => {}
        Err(error) => return Err(error),
        _ => {}
    };

    Ok(code_hash)
}
