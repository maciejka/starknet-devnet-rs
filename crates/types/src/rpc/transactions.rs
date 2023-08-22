use broadcasted_declare_transaction_v1::BroadcastedDeclareTransactionV1;
use broadcasted_declare_transaction_v2::BroadcastedDeclareTransactionV2;
use broadcasted_deploy_account_transaction::BroadcastedDeployAccountTransaction;
use broadcasted_invoke_transaction_v1::BroadcastedInvokeTransactionV1;
use declare_transaction_v0v1::DeclareTransactionV0V1;
use declare_transaction_v2::DeclareTransactionV2;
use deploy_account_transaction::DeployAccountTransaction;
use deploy_transaction::DeployTransaction;
use invoke_transaction_v1::InvokeTransactionV1;
use serde::{Deserialize, Serialize};
use starknet_api::block::BlockNumber;
use starknet_api::transaction::{EthAddress, Fee};
use starknet_rs_core::types::{BlockId, TransactionStatus};

use crate::contract_address::ContractAddress;
use crate::emitted_event::Event;
use crate::felt::{
    BlockHash, Calldata, EntryPointSelector, Felt, Nonce, TransactionHash, TransactionSignature,
    TransactionVersion,
};

pub mod broadcasted_declare_transaction_v1;
pub mod broadcasted_declare_transaction_v2;
pub mod broadcasted_deploy_account_transaction;
pub mod broadcasted_invoke_transaction_v1;

pub mod declare_transaction_v0v1;
pub mod declare_transaction_v2;
pub mod deploy_account_transaction;
pub mod deploy_transaction;
pub mod invoke_transaction_v1;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Transactions {
    Hashes(Vec<TransactionHash>),
    Full(Vec<TransactionWithType>),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct TransactionWithType {
    pub r#type: TransactionType,
    #[serde(flatten)]
    pub transaction: Transaction,
}

impl TransactionWithType {
    pub fn max_fee(&self) -> Fee {
        self.transaction.max_fee()
    }

    pub fn get_transaction_hash(&self) -> &TransactionHash {
        self.transaction.get_transaction_hash()
    }

    // TODO: maybe move to StarknetTransaction
    pub fn create_common_receipt(
        &self,
        transaction_events: &[Event],
        block_hash: &BlockHash,
        block_number: BlockNumber,
    ) -> CommonTransactionReceipt {
        let output = TransactionOutput {
            actual_fee: self.max_fee(),
            messages_sent: Vec::new(),
            events: transaction_events.to_vec(),
        };

        CommonTransactionReceipt {
            r#type: self.r#type,
            transaction_hash: *self.get_transaction_hash(),
            block_hash: *block_hash,
            block_number,
            output,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize, Default)]
pub enum TransactionType {
    #[serde(rename(deserialize = "DECLARE", serialize = "DECLARE"))]
    Declare,
    #[serde(rename(deserialize = "DEPLOY", serialize = "DEPLOY"))]
    Deploy,
    #[serde(rename(deserialize = "DEPLOY_ACCOUNT", serialize = "DEPLOY_ACCOUNT"))]
    DeployAccount,
    #[serde(rename(deserialize = "INVOKE", serialize = "INVOKE"))]
    #[default]
    Invoke,
    #[serde(rename(deserialize = "L1_HANDLER", serialize = "L1_HANDLER"))]
    L1Handler,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Transaction {
    Declare(DeclareTransaction),
    DeployAccount(DeployAccountTransaction),
    Deploy(DeployTransaction),
    Invoke(InvokeTransaction),
    L1Handler(L1HandlerTransaction),
}

impl Transaction {
    pub fn get_type(&self) -> TransactionType {
        match self {
            Transaction::Declare(_) => TransactionType::Declare,
            Transaction::DeployAccount(_) => TransactionType::DeployAccount,
            Transaction::Deploy(_) => TransactionType::Deploy,
            Transaction::Invoke(_) => TransactionType::Invoke,
            Transaction::L1Handler(_) => TransactionType::L1Handler,
        }
    }

    pub fn max_fee(&self) -> Fee {
        match self {
            Transaction::Declare(tx) => tx.max_fee(),
            Transaction::DeployAccount(tx) => tx.max_fee(),
            Transaction::Deploy(tx) => tx.max_fee(),
            Transaction::Invoke(tx) => tx.max_fee(),
            Transaction::L1Handler(tx) => tx.max_fee(),
        }
    }

    pub fn get_transaction_hash(&self) -> &TransactionHash {
        match self {
            Transaction::Declare(tx) => tx.get_transaction_hash(),
            Transaction::L1Handler(tx) => tx.get_transaction_hash(),
            Transaction::DeployAccount(tx) => tx.get_transaction_hash(),
            Transaction::Invoke(tx) => tx.get_transaction_hash(),
            Transaction::Deploy(tx) => tx.get_transaction_hash(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeclareTransaction {
    Version0(DeclareTransactionV0V1),
    Version1(DeclareTransactionV0V1),
    Version2(DeclareTransactionV2),
}

impl DeclareTransaction {
    pub fn max_fee(&self) -> Fee {
        match self {
            DeclareTransaction::Version0(tx) => tx.max_fee(),
            DeclareTransaction::Version1(tx) => tx.max_fee(),
            DeclareTransaction::Version2(tx) => tx.max_fee(),
        }
    }

    pub fn get_transaction_hash(&self) -> &TransactionHash {
        match self {
            DeclareTransaction::Version0(tx) => tx.get_transaction_hash(),
            DeclareTransaction::Version1(tx) => tx.get_transaction_hash(),
            DeclareTransaction::Version2(tx) => tx.get_transaction_hash(),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct InvokeTransactionV0 {
    pub transaction_hash: TransactionHash,
    pub max_fee: Fee,
    pub version: TransactionVersion,
    pub signature: TransactionSignature,
    pub nonce: Nonce,
    pub contract_address: ContractAddress,
    pub entry_point_selector: EntryPointSelector,
    pub calldata: Calldata,
}

impl InvokeTransactionV0 {
    pub fn get_max_fee(&self) -> Fee {
        self.max_fee
    }

    pub fn get_transaction_hash(&self) -> &TransactionHash {
        &self.transaction_hash
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InvokeTransaction {
    Version0(InvokeTransactionV0),
    Version1(InvokeTransactionV1),
}

impl InvokeTransaction {
    pub fn max_fee(&self) -> Fee {
        match self {
            InvokeTransaction::Version0(tx) => tx.get_max_fee(),
            InvokeTransaction::Version1(tx) => tx.max_fee(),
        }
    }

    pub fn get_transaction_hash(&self) -> &TransactionHash {
        match self {
            InvokeTransaction::Version0(tx) => tx.get_transaction_hash(),
            InvokeTransaction::Version1(tx) => tx.get_transaction_hash(),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct L1HandlerTransaction {
    pub transaction_hash: TransactionHash,
    pub version: TransactionVersion,
    pub nonce: Nonce,
    pub contract_address: ContractAddress,
    pub entry_point_selector: EntryPointSelector,
    pub calldata: Calldata,
}

impl L1HandlerTransaction {
    pub fn max_fee(&self) -> Fee {
        Fee(0)
    }

    // TODO: if tyoe implementing copy shall be returned by reference?
    pub fn get_transaction_hash(&self) -> &TransactionHash {
        &self.transaction_hash
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransactionReceiptWithStatus {
    pub status: TransactionStatus,
    #[serde(flatten)]
    pub receipt: TransactionReceipt,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionReceipt {
    Deploy(DeployTransactionReceipt),
    Common(CommonTransactionReceipt),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeployTransactionReceipt {
    #[serde(flatten)]
    pub common: CommonTransactionReceipt,
    pub contract_address: ContractAddress,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommonTransactionReceipt {
    pub transaction_hash: TransactionHash,
    pub r#type: TransactionType,
    pub block_hash: BlockHash,
    pub block_number: BlockNumber,
    #[serde(flatten)]
    pub output: TransactionOutput,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub actual_fee: Fee,
    pub messages_sent: Vec<MessageToL1>,
    pub events: Vec<Event>,
}

pub type L2ToL1Payload = Vec<Felt>;

/// An L2 to L1 message.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct MessageToL1 {
    pub from_address: ContractAddress,
    pub to_address: EthAddress,
    pub payload: L2ToL1Payload,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EventFilter {
    pub from_block: Option<BlockId>,
    pub to_block: Option<BlockId>,
    pub address: Option<ContractAddress>,
    pub keys: Option<Vec<Vec<Felt>>>,
    pub continuation_token: Option<String>,
    pub chunk_size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct EventsChunk {
    pub events: Vec<crate::emitted_event::EmittedEvent>,
    pub continuation_token: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct FunctionCall {
    pub contract_address: ContractAddress,
    pub entry_point_selector: EntryPointSelector,
    pub calldata: Calldata,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct BroadcastedTransactionCommon {
    pub max_fee: Fee,
    pub version: TransactionVersion,
    pub signature: TransactionSignature,
    pub nonce: Nonce,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct BroadcastedTransactionWithType {
    pub r#type: TransactionType,
    #[serde(flatten)]
    pub transaction: BroadcastedTransaction,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BroadcastedTransaction {
    Invoke(BroadcastedInvokeTransaction),
    Declare(BroadcastedDeclareTransaction),
    DeployAccount(BroadcastedDeployAccountTransaction),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BroadcastedInvokeTransaction {
    V0(BroadcastedInvokeTransactionV0),
    V1(BroadcastedInvokeTransactionV1),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BroadcastedDeclareTransaction {
    V1(Box<BroadcastedDeclareTransactionV1>),
    V2(Box<BroadcastedDeclareTransactionV2>),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct BroadcastedInvokeTransactionV0 {
    #[serde(flatten)]
    pub common: BroadcastedTransactionCommon,
    pub contract_address: ContractAddress,
    pub entry_point_selector: EntryPointSelector,
    pub calldata: Calldata,
}