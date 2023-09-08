use gclient::{
    errors::{Gear, ModuleError},
    Error as GclientError, EventListener, EventProcessor, GearApi, Result,
};
use gstd::{
    prelude::{fmt::Debug, *},
    ActorId,
};
use primitive_types::H256;
use supply_chain_io::*;
use supply_chain_state::{WASM_BINARY, WASM_EXPORTS};

pub const FT_MAIN: &str = "../target/wasm32-unknown-unknown/debug/sharded_fungible_token.opt.wasm";
pub const FT_STORAGE: &str =
    "../target/wasm32-unknown-unknown/debug/sharded_fungible_token_storage.opt.wasm";
pub const FT_LOGIC: &str =
    "../target/wasm32-unknown-unknown/debug/sharded_fungible_token_logic.opt.wasm";
pub const NFT_BINARY: &str = "../target/wasm32-unknown-unknown/debug/non_fungible_token.opt.wasm";

pub struct Client {
    client: GearApi,
    listener: EventListener,
}

impl Client {
    pub async fn global() -> Result<Self> {
        let client = GearApi::gear().await?;
        let listener = client.subscribe().await?;

        Ok(Self { client, listener })
    }

    pub fn login(mut self, suri: impl AsRef<str>) -> Result<Self> {
        self.client = self.client.with(suri)?;

        Ok(self)
    }

    pub async fn local() -> Result<Self> {
        let client = GearApi::dev_from_path("../target/tmp/gear").await?;
        let listener = client.subscribe().await?;

        Ok(Self { client, listener })
    }

    pub async fn upload_code(&self, path: &str) -> Result<H256> {
        let code_id = match self.client.upload_code_by_path(path).await {
            Ok((code_id, _)) => code_id.into(),
            Err(GclientError::Module(ModuleError::Gear(Gear::CodeAlreadyExists))) => {
                sp_core_hashing::blake2_256(&gclient::code_from_os(path)?)
            }
            Err(other_error) => return Err(other_error),
        };

        println!("Uploaded `{path}`.");

        Ok(code_id.into())
    }

    pub async fn upload_program(&mut self, path: &str, payload: impl Encode) -> Result<[u8; 32]> {
        let (message_id, program_id) = self
            .common_upload_program(gclient::code_from_os(path)?, payload)
            .await?;

        assert!(self
            .listener
            .message_processed(message_id.into())
            .await?
            .succeed());
        println!("Initialized `{path}`.");

        Ok(program_id)
    }

    pub async fn upload_program_and_wait_reply<R: Decode>(
        &mut self,
        code: Vec<u8>,
        payload: impl Encode,
    ) -> Result<([u8; 32], R)> {
        let (message_id, program_id) = self.common_upload_program(code, payload).await?;
        let (_, raw_reply, _) = self.listener.reply_bytes_on(message_id.into()).await?;
        let reply = decode(
            raw_reply.expect("initialization failed, received an error message instead of a reply"),
        )?;

        Ok((program_id, reply))
    }

    async fn common_upload_program(
        &self,
        code: Vec<u8>,
        payload: impl Encode,
    ) -> Result<([u8; 32], [u8; 32])> {
        let encoded_payload = payload.encode();
        let gas_limit = self
            .client
            .calculate_upload_gas(None, code.clone(), encoded_payload, 0, true)
            .await?
            .min_limit;
        let (message_id, program_id, _) = self
            .client
            .upload_program(
                code,
                gclient::now_micros().to_le_bytes(),
                payload,
                gas_limit * 2,
                0,
            )
            .await?;

        Ok((message_id.into(), program_id.into()))
    }

    pub async fn send_message<R: Decode>(
        &mut self,
        destination: [u8; 32],
        payload: impl Encode + Debug,
    ) -> Result<R> {
        Ok(self
            .send_message_with_custom_limit(destination, payload, |gas| gas * 2)
            .await?
            .expect("action failed, received an error message instead of a reply"))
    }

    pub async fn send_message_with_custom_limit<R: Decode>(
        &mut self,
        destination: [u8; 32],
        payload: impl Encode + Debug,
        modify_gas_limit: fn(u64) -> u64,
    ) -> Result<Result<R, String>> {
        let encoded_payload = payload.encode();
        let destination = destination.into();

        let gas_limit = self
            .client
            .calculate_handle_gas(None, destination, encoded_payload, 0, true)
            .await?
            .min_limit;
        let modified_gas_limit = modify_gas_limit(gas_limit);

        println!("Sending a payload: `{payload:?}`.");
        println!("Calculated gas limit: {gas_limit}.");
        println!("Modified gas limit: {modified_gas_limit}.");

        let (message_id, _) = self
            .client
            .send_message(destination, payload, modified_gas_limit, 0, false)
            .await?;

        println!("Sending completed.");

        let (_, raw_reply, _) = self.listener.reply_bytes_on(message_id).await?;

        Ok(match raw_reply {
            Ok(raw_reply) => Ok(decode(raw_reply)?),
            Err(error) => Err(error),
        })
    }

    pub async fn send_message_with_insufficient_gas(
        &mut self,
        destination: [u8; 32],
        payload: impl Encode + Debug,
    ) -> Result<String> {
        Ok(self
            .send_message_with_custom_limit::<()>(destination, payload, |gas| gas - gas / 100)
            .await?
            .expect_err("received a reply instead of an error message"))
    }

    pub async fn send_message_for_sc(
        &mut self,
        destination: [u8; 32],
        payload: impl Encode + Debug,
    ) -> Result<Result<Event, Error>> {
        self.send_message(destination, payload).await
    }

    pub async fn is_action_cached(
        &self,
        supply_chain_actor_id: [u8; 32],
        action: Action,
    ) -> Result<bool> {
        self.client
            .read_state_using_wasm::<_, bool>(
                supply_chain_actor_id.into(),
                vec![],
                WASM_EXPORTS[7],
                WASM_BINARY.into(),
                Some((ActorId::from(ALICE), action.clone().action)),
            )
            .await
    }
}

pub const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

fn decode<T: Decode>(payload: Vec<u8>) -> Result<T> {
    Ok(T::decode(&mut payload.as_slice())?)
}
