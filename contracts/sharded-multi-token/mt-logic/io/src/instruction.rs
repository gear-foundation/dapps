use gmeta::{Decode, Encode, TypeInfo};
use gstd::{msg, ActorId};
use mt_storage_io::{MTStorageAction, MTStorageEvent};
use primitive_types::H256;

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum InstructionState {
    ScheduledRun,
    ScheduledAbort,
    RunWithError,
    Finished,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Instruction {
    state: InstructionState,
    address: ActorId,
    transaction: MTStorageAction,
    compensation: Option<MTStorageAction>,
}

impl Instruction {
    /// Create a new instruction from a given transaction and a compensation
    pub fn new(
        address: ActorId,
        transaction: MTStorageAction,
        compensation: Option<MTStorageAction>,
    ) -> Self {
        Instruction {
            state: InstructionState::ScheduledRun,
            address,
            transaction,
            compensation,
        }
    }

    pub async fn start(&mut self) -> Result<(), ()> {
        match self.state {
            InstructionState::ScheduledRun => {
                let result = msg::send_for_reply_as::<_, MTStorageEvent>(
                    self.address,
                    self.transaction.clone(),
                    0,
                )
                .expect("Error in sending a message in instruction")
                .await;

                match result {
                    Ok(MTStorageEvent::Ok) => {
                        self.state = InstructionState::ScheduledAbort;
                        Ok(())
                    }
                    _ => {
                        self.state = InstructionState::RunWithError;
                        Err(())
                    }
                }
            }
            InstructionState::RunWithError => Err(()),
            _ => Ok(()),
        }
    }

    pub async fn abort(&mut self) -> Result<(), ()> {
        match self.state {
            InstructionState::ScheduledAbort => {
                let result = msg::send_for_reply_as::<_, MTStorageEvent>(
                    self.address,
                    self.compensation
                        .as_ref()
                        .expect("No compensation for that instruction"),
                    0,
                )
                .expect("Error in sending a compensation message in instruction")
                .await;

                match result {
                    Ok(MTStorageEvent::Ok) => {
                        self.state = InstructionState::Finished;
                        Ok(())
                    }
                    _ => Err(()),
                }
            }
            InstructionState::Finished => Ok(()),
            _ => Err(()),
        }
    }
}

pub fn create_decrease_instruction(
    transaction_hash: H256,
    sender_storage: &ActorId,
    token_id: u128,
    msg_source: &ActorId,
    account: &ActorId,
    amount: u128,
) -> Instruction {
    Instruction::new(
        *sender_storage,
        MTStorageAction::DecreaseBalance {
            transaction_hash,
            token_id,
            msg_source: *msg_source,
            account: *account,
            amount,
        },
        Some(MTStorageAction::IncreaseBalance {
            transaction_hash,
            token_id,
            account: *account,
            amount,
        }),
    )
}

pub fn create_increase_instruction(
    transaction_hash: H256,
    recipient_storage: &ActorId,
    token_id: u128,
    account: &ActorId,
    amount: u128,
) -> Instruction {
    Instruction::new(
        *recipient_storage,
        MTStorageAction::IncreaseBalance {
            transaction_hash,
            token_id,
            account: *account,
            amount,
        },
        None,
    )
}
