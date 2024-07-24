use blake2_rfc::blake2b;
use gclient::GearApi;
use gstd::{ActorId, Encode};

pub const HASH_LENGTH: usize = 32;
pub type Hash = [u8; HASH_LENGTH];
pub const USERS: [&str; 5] = ["//Mike", "//John", "//Alex", "//Peter", "//Alice"];
pub const USERS_FUND: u128 = 1_000_000_000_000_000;
pub const SELLER: &str = "//Markus";
pub const BUYER: &str = "//Jim";
pub const TREASURY: &str = "//Treasury";
pub const TREASURY_FEE: u16 = 3;
pub const TOKEN_ID: u128 = 0;
pub const NFT_PRICE: u128 = 1_000_000_000_000_000;
pub const BID_PERIOD: u64 = 3_600_000;
pub const DURATION: u64 = 86_400_000;

static mut API: Option<GearApi> = None;

pub async fn init_gear_api_from_path() -> gclient::Result<GearApi> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    unsafe { API = Some(api.clone()) };

    Ok(api)
}

pub fn gear_api_from_path() -> GearApi {
    unsafe { API.as_ref().unwrap().clone() }
}

pub fn get_current_actor_id(api: &GearApi) -> ActorId {
    ActorId::new(*api.account_id().clone().as_ref())
}

pub async fn get_user_to_actor_id(user: impl AsRef<str>) -> gclient::Result<ActorId> {
    let api = gear_api_from_path().with(user)?;
    let actor_id = ActorId::new(*api.account_id().clone().as_ref());

    Ok(actor_id)
}

pub async fn init(api: &GearApi) -> gclient::Result<(ActorId, ActorId, ActorId)> {
    for user in &USERS[..4] {
        let user_id = get_user_to_actor_id(user).await?;
        api.transfer_keep_alive(
            user_id
                .encode()
                .as_slice()
                .try_into()
                .expect("Unexpected invalid `ProgramId`."),
            USERS_FUND,
        )
        .await?;
    }

    let seller_id = get_user_to_actor_id(SELLER).await?;
    let buyer_id = get_user_to_actor_id(BUYER).await?;
    let treasury_id = get_user_to_actor_id(TREASURY).await?;

    api.transfer_keep_alive(
        seller_id
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ProgramId`."),
        api.total_balance(api.account_id()).await? / 2,
    )
    .await?;
    api.transfer_keep_alive(
        buyer_id
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ProgramId`."),
        api.total_balance(api.account_id()).await? / 2,
    )
    .await?;
    api.transfer_keep_alive(
        treasury_id
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ProgramId`."),
        USERS_FUND,
    )
    .await?;

    let ft_contract = super::ft::init(api).await?;
    let nft_contract = super::nft::init(api).await?;

    super::nft::add_minter(
        api,
        &mut api.subscribe().await?,
        nft_contract,
        0,
        get_user_to_actor_id(SELLER).await?,
    )
    .await?;

    {
        let seller_api = gear_api_from_path().with(SELLER)?;
        let mut listener = seller_api.subscribe().await?;
        super::nft::mint(&seller_api, &mut listener, &nft_contract, 0).await?;
    }

    let admin_id = get_current_actor_id(api);
    let treasury_id = get_user_to_actor_id(TREASURY).await?;
    let market_contract = super::marketplace::init(api, &admin_id, &treasury_id).await?;

    {
        let buyer_api = gear_api_from_path().with(BUYER)?;
        let mut listener = buyer_api.subscribe().await?;
        super::ft::approve(
            &buyer_api,
            &mut listener,
            &ft_contract,
            0,
            &market_contract,
            NFT_PRICE,
        )
        .await?;
    }

    {
        let seller_api = gear_api_from_path().with(SELLER)?;
        let mut listener = seller_api.subscribe().await?;
        super::nft::approve(
            &seller_api,
            &mut listener,
            &nft_contract,
            1,
            &market_contract,
            TOKEN_ID.into(),
        )
        .await?;
    }
    let mut listener = api.subscribe().await?;
    super::marketplace::add_ft_contract(api, &mut listener, &market_contract, &ft_contract, false)
        .await?;
    super::marketplace::add_nft_contract(
        api,
        &mut listener,
        &market_contract,
        &nft_contract,
        false,
    )
    .await?;

    Ok((ft_contract, nft_contract, market_contract))
}

pub async fn upload_with_code_hash(
    api: &GearApi,
    wasm_path: impl AsRef<str>,
) -> gclient::Result<Hash> {
    let mut code_hash: Hash = Default::default();
    let wasm_code = gclient::code_from_os(wasm_path.as_ref())?;

    code_hash[..].copy_from_slice(blake2b::blake2b(HASH_LENGTH, &[], &wasm_code).as_bytes());

    api.upload_code(wasm_code).await?;

    Ok(code_hash)
}
