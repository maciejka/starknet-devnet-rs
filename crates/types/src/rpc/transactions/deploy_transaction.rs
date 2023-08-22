use serde::{Deserialize, Serialize};
use starknet_api::transaction::Fee;

use crate::felt::{Calldata, ClassHash, ContractAddressSalt, TransactionHash, TransactionVersion};

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct DeployTransaction {
    pub transaction_hash: TransactionHash,
    pub version: TransactionVersion,
    pub class_hash: ClassHash,
    pub contract_address_salt: ContractAddressSalt,
    pub constructor_calldata: Calldata,
}

impl DeployTransaction {
    pub fn max_fee(&self) -> Fee {
        // TODO: check
        Fee(0)
    }

    pub fn get_transaction_hash(&self) -> &TransactionHash {
        &self.transaction_hash
    }
}
