use crate::H256;
use ft_storage_io::{FTStorageAction, FTStorageEvent};
use gstd::{msg, prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum InstructionState {
    ScheduledRun,
    ScheduledAbort,
    RunWithError,
    Finished,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Instruction {
    pub state: InstructionState,
    pub address: ActorId,
    pub transaction: FTStorageAction,
    pub compensation: Option<FTStorageAction>,
}

impl Instruction {
    /// Create a new instruction from a given transaction and a compensation
    pub fn new(
        address: ActorId,
        transaction: FTStorageAction,
        compensation: Option<FTStorageAction>,
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
                let result =
                    msg::send_for_reply_as::<_, FTStorageEvent>(self.address, self.transaction, 0)
                        .expect("Error in sending a message in instruction")
                        .await;
                match result {
                    Ok(FTStorageEvent::Ok) => {
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
                let result = msg::send_for_reply_as::<_, FTStorageEvent>(
                    self.address,
                    self.compensation
                        .expect("No compensation for that instruction"),
                    0,
                )
                .expect("Error in sending a compensation message in instruction")
                .await;
                match result {
                    Ok(FTStorageEvent::Ok) => {
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
    msg_source: &ActorId,
    sender_storage: &ActorId,
    sender: &ActorId,
    amount: u128,
) -> Instruction {
    Instruction::new(
        *sender_storage,
        FTStorageAction::DecreaseBalance {
            transaction_hash,
            msg_source: *msg_source,
            account: *sender,
            amount,
        },
        Some(FTStorageAction::IncreaseBalance {
            transaction_hash,
            account: *sender,
            amount,
        }),
    )
}

pub fn create_increase_instruction(
    transaction_hash: H256,
    recipient_storage: &ActorId,
    recipient: &ActorId,
    amount: u128,
) -> Instruction {
    Instruction::new(
        *recipient_storage,
        FTStorageAction::IncreaseBalance {
            transaction_hash,
            account: *recipient,
            amount,
        },
        None,
    )
}
