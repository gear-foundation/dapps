use blake2_rfc::blake2b;
use gclient::GearApi;
use gstd::{prelude::*, ActorId};

pub const HASH_LENGTH: usize = 32;
pub type Hash = [u8; HASH_LENGTH];
pub const APPLICANTS: [&str; 6] = ["//Jack", "//Mike", "//Ben", "//Nick", "//John", "//Paul"];

pub fn get_current_actor_id(api: &GearApi) -> ActorId {
    ActorId::new(*api.account_id().clone().as_ref())
}

pub async fn get_user_to_actor_id(user: impl AsRef<str>) -> gclient::Result<ActorId> {
    let api = GearApi::dev().await?.with(user)?;
    let actor_id = ActorId::new(*api.account_id().clone().as_ref());

    Ok(actor_id)
}

pub async fn fund_applicants(api: &GearApi) -> gclient::Result<()> {
    let fund_amount = api.total_balance(api.account_id()).await? / (APPLICANTS.len() as u128 + 1);

    for applicant in APPLICANTS {
        let id = get_user_to_actor_id(applicant).await?.encode();
        api.transfer(id.as_slice().into(), fund_amount).await?;
    }

    Ok(())
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
        Err(gclient::Error::Subxt(subxt::Error::Runtime(subxt::error::DispatchError::Module(
            subxt::error::ModuleError {
                error_data:
                    subxt::error::ModuleErrorData {
                        pallet_index: 104,
                        error: [6, 0, 0, 0],
                    },
                ..
            },
        )))) => {}
        Err(error) => return Err(error),
        _ => {}
    };

    Ok(code_hash)
}
