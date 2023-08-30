use crate::*;

#[derive(Default)]
pub struct TxManager {
    pub txs: BTreeMap<MessageId, Tx>,
    // mapping from send message ID to processing message ID
    pub msg_sent_to_msg: BTreeMap<MessageId, MessageId>,
}

impl TxManager {
    pub fn set_tx(&mut self, msg: &RMRKAction) -> &mut Tx {
        self.txs.entry(msg::id()).or_insert_with(|| Tx {
            msg: msg.clone(),
            state: TxState::Initial,
            data: None,
            processing_msg_payload: None,
        })
    }

    pub fn set_tx_state(&mut self, tx_state: TxState, msg_id: MessageId) {
        let current_msg_id = msg::id();
        self.txs.entry(current_msg_id).and_modify(|tx| {
            tx.state = tx_state;
        });
        self.msg_sent_to_msg.insert(msg_id, current_msg_id);
    }

    pub fn set_tx_data(&mut self, tx_data: Vec<u8>) {
        let current_msg_id = msg::id();
        self.txs.entry(current_msg_id).and_modify(|tx| {
            tx.data = Some(tx_data);
        });
    }

    pub fn set_processing_msg(&mut self, payload: Vec<u8>) {
        let current_msg_id = msg::id();
        self.txs.entry(current_msg_id).and_modify(|tx| {
            tx.processing_msg_payload = Some(payload);
        });
    }

    pub fn get_decoded_data<T>(&self) -> Result<T, RMRKError>
    where
        T: Decode,
    {
        let current_msg_id = msg::id();
        if let Some(tx) = self.txs.get(&current_msg_id) {
            if let Some(data) = &tx.data {
                match <T>::decode(&mut &data[..]) {
                    Ok(t) => return Ok(t),
                    Err(_) => return Err(RMRKError::UnknownError),
                }
            }
        }
        Err(RMRKError::UnknownError)
    }

    pub fn get_payload<T>(&self) -> Result<T, RMRKError>
    where
        T: Decode,
    {
        let current_msg_id = msg::id();
        if let Some(tx) = self.txs.get(&current_msg_id) {
            if let Some(payload) = &tx.processing_msg_payload {
                match <T>::decode(&mut &payload[..]) {
                    Ok(t) => return Ok(t),
                    Err(_) => return Err(RMRKError::UnknownError),
                }
            }
        }
        Err(RMRKError::UnknownError)
    }

    pub fn get_state(&self, msg_id: MessageId) -> TxState {
        let tx = self.txs.get(&msg_id).expect("Cant be None");
        tx.state.clone()
    }

    pub fn tx_does_not_exist(&self) -> bool {
        if self.txs.contains_key(&msg::id()) {
            return false;
        }
        true
    }

    pub fn check_for_error(&mut self) -> Result<(), RMRKError> {
        let current_msg_id = msg::id();
        if let Some(tx) = self.txs.get(&current_msg_id) {
            match tx.state.clone() {
                TxState::Error(error) => {
                    return Err(error);
                }
                _ => return Ok(()),
            }
        }
        Ok(())
    }
}
