use serde::{Deserialize, Serialize};

use crate::contract_address::ContractAddress;
use crate::felt::{ClassHash, CompiledClassHash, Felt, TransactionHash};
use crate::rpc::transactions::declare_transaction_v2::DeclareTransactionV2;
use crate::rpc::transactions::BroadcastedTransactionCommon;
use crate::serde_helpers::rpc_sierra_contract_class_to_sierra_contract_class::deserialize_to_sierra_contract_class;

use crate::error::DevnetResult;
use starknet_in_rust::transaction::DeclareV2 as SirDeclareV2;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct BroadcastedDeclareTransactionV2 {
    #[serde(flatten)]
    pub common: BroadcastedTransactionCommon,
    #[serde(deserialize_with = "deserialize_to_sierra_contract_class")]
    pub contract_class: starknet_in_rust::SierraContractClass,
    pub sender_address: ContractAddress,
    pub compiled_class_hash: CompiledClassHash,
}

impl BroadcastedDeclareTransactionV2 {
    pub fn compile_declare(
        &self,
        class_hash: &ClassHash,
        transaction_hash: &TransactionHash,
    ) -> DeclareTransactionV2 {
        DeclareTransactionV2 {
            class_hash: class_hash.clone(),
            compiled_class_hash: self.compiled_class_hash,
            sender_address: self.sender_address,
            nonce: self.common.nonce,
            max_fee: self.common.max_fee,
            version: self.common.version,
            transaction_hash: transaction_hash.clone(),
            signature: self.common.signature.clone(),
        }
    }

    pub fn compile_sir_declare(&self, chain_id: &Felt) -> DevnetResult<SirDeclareV2> {
        Ok(SirDeclareV2::new(
            &self.contract_class,
            None,
            self.compiled_class_hash.into(),
            chain_id.into(),
            self.sender_address.into(),
            self.common.max_fee.0,
            self.common.version.into(),
            self.common.signature.iter().map(|felt| felt.into()).collect(),
            self.common.nonce.into(),
        )?)
    }
}
