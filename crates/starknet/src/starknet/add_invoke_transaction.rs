use starknet_in_rust::definitions::constants::INITIAL_GAS_COST;
use starknet_in_rust::transaction::error::TransactionError;
use starknet_types::felt::TransactionHash;
use starknet_types::rpc::transactions::broadcasted_invoke_transaction_v1::BroadcastedInvokeTransactionV1;
use starknet_types::rpc::transactions::{
    InvokeTransaction, Transaction, TransactionType, TransactionWithType,
};
use starknet_types::traits::HashProducer;

use super::Starknet;
use crate::error::{self, DevnetResult};
use crate::transactions::StarknetTransaction;

pub fn add_invoke_transcation_v1(
    starknet: &mut Starknet,
    broadcasted_invoke_transaction: BroadcastedInvokeTransactionV1,
) -> DevnetResult<TransactionHash> {
    if broadcasted_invoke_transaction.common.max_fee.0 == 0 {
        return Err(error::Error::TransactionError(TransactionError::FeeError(
            "For invoke transaction, max fee cannot be 0".to_string(),
        )));
    }

    let sir_invoke_function = broadcasted_invoke_transaction
        .create_sir_invoke_function(&starknet.config.chain_id.to_felt().into())?;
    let transaction_hash = sir_invoke_function.hash_value().into();
    let invoke_transaction =
        broadcasted_invoke_transaction.create_invoke_transaction(&transaction_hash);

    let transaction_with_type = TransactionWithType {
        r#type: TransactionType::Invoke,
        transaction: Transaction::Invoke(InvokeTransaction::Version1(invoke_transaction)),
    };

    let state_before_txn = starknet.state.pending_state.clone();

    match sir_invoke_function.execute(
        &mut starknet.state.pending_state,
        &starknet.block_context,
        INITIAL_GAS_COST,
    ) {
        Ok(tx_info) => {
            starknet.handle_successful_transaction(
                &transaction_hash,
                transaction_with_type,
                tx_info,
            )?;
        }
        Err(tx_err) => {
            let transaction_to_add =
                StarknetTransaction::create_rejected(transaction_with_type, tx_err);

            starknet.transactions.insert(&transaction_hash, transaction_to_add);
            // Revert to previous pending state
            starknet.state.pending_state = state_before_txn;
        }
    }

    Ok(transaction_hash)
}

#[cfg(test)]
mod tests {
    use starknet_api::transaction::Fee;
    use starknet_in_rust::services::api::contract_classes::deprecated_contract_class::ContractClass as StarknetInRustContractClass;
    use starknet_in_rust::EntryPointType;
    use starknet_rs_core::types::TransactionStatus;
    use starknet_rs_core::utils::get_selector_from_name;
    use starknet_types::contract_address::ContractAddress;
    use starknet_types::contract_class::{Cairo0ContractClass, ContractClass};
    use starknet_types::contract_storage_key::ContractStorageKey;
    use starknet_types::felt::Felt;
    use starknet_types::rpc::transactions::broadcasted_invoke_transaction_v1::BroadcastedInvokeTransactionV1;
    use starknet_types::traits::HashProducer;

    use crate::account::Account;
    use crate::constants::{self, DEVNET_DEFAULT_CHAIN_ID};
    use crate::starknet::{predeployed, Starknet};
    use crate::traits::{Accounted, Deployed, HashIdentifiedMut, StateChanger, StateExtractor};
    use crate::utils::get_storage_var_address;
    use crate::utils::test_utils::{
        cairo_0_account_without_validations, dummy_cairo_0_contract_class, dummy_contract_address,
        dummy_felt, get_bytes_from_u32,
    };

    fn test_invoke_transaction_v1(
        account_address: ContractAddress,
        contract_address: ContractAddress,
        function_selector: Felt,
        param: Felt,
        nonce: u128,
    ) -> BroadcastedInvokeTransactionV1 {
        let calldata = vec![
            Felt::from(contract_address), // contract address
            function_selector,            // function selector
            Felt::from(1),                // calldata len
            param,                        // calldata
        ];

        BroadcastedInvokeTransactionV1::new(
            account_address,
            Fee(10000),
            &vec![],
            Felt::from(nonce),
            &calldata,
            Felt::from(1),
        )
    }

    #[test]
    fn invoke_transaction_successful_execution() {
        let (mut starknet, account_address, contract_address, increase_balance_selector, _) =
            setup();

        let invoke_transaction = test_invoke_transaction_v1(
            account_address,
            contract_address,
            increase_balance_selector,
            Felt::from(10),
            0,
        );

        let transaction_hash = starknet.add_invoke_transaction_v1(invoke_transaction).unwrap();

        let transaction = starknet.transactions.get_by_hash_mut(&transaction_hash).unwrap();

        assert_eq!(transaction.status, TransactionStatus::AcceptedOnL2);
    }

    #[test]
    fn invoke_transaction_successfully_changes_storage() {
        let (
            mut starknet,
            account_address,
            contract_address,
            increase_balance_selector,
            balance_var_storage_address,
        ) = setup();

        let invoke_transaction = test_invoke_transaction_v1(
            account_address,
            contract_address,
            increase_balance_selector,
            Felt::from(10),
            0,
        );

        // invoke transaction
        let transaction_hash = starknet.add_invoke_transaction_v1(invoke_transaction).unwrap();
        let transaction = starknet.transactions.get_by_hash_mut(&transaction_hash).unwrap();
        assert_eq!(transaction.status, TransactionStatus::AcceptedOnL2);

        // check storage
        assert_eq!(
            starknet.state.get_storage(balance_var_storage_address).unwrap(),
            Felt::from(10)
        );

        let invoke_transaction = test_invoke_transaction_v1(
            account_address,
            contract_address,
            increase_balance_selector,
            Felt::from(15),
            1,
        );

        // invoke transaction again
        let transaction_hash = starknet.add_invoke_transaction_v1(invoke_transaction).unwrap();
        let transaction = starknet.transactions.get_by_hash_mut(&transaction_hash).unwrap();

        assert_eq!(transaction.status, TransactionStatus::AcceptedOnL2);
        assert_eq!(
            starknet.state.get_storage(balance_var_storage_address).unwrap(),
            Felt::from(25)
        );
    }

    #[test]
    fn invoke_transaction_with_max_fee_zero_should_return_error() {
        let invoke_transaction = BroadcastedInvokeTransactionV1::new(
            dummy_contract_address(),
            Fee(0),
            &vec![],
            dummy_felt(),
            &vec![],
            Felt::from(1),
        );

        let result = Starknet::default().add_invoke_transaction_v1(invoke_transaction);

        assert!(result.is_err());
        match result.err().unwrap() {
            crate::error::Error::TransactionError(
                starknet_in_rust::transaction::error::TransactionError::FeeError(msg),
            ) => assert_eq!(msg, "For invoke transaction, max fee cannot be 0"),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn invoke_transaction_should_fail_if_same_nonce_supplied() {
        let (mut starknet, account_address, contract_address, increase_balance_selector, _) =
            setup();

        let invoke_transaction = test_invoke_transaction_v1(
            account_address,
            contract_address,
            increase_balance_selector,
            Felt::from(10),
            0,
        );

        let transaction_hash =
            starknet.add_invoke_transaction_v1(invoke_transaction.clone()).unwrap();
        let transaction = starknet.transactions.get_by_hash_mut(&transaction_hash).unwrap();
        assert_eq!(transaction.status, TransactionStatus::AcceptedOnL2);

        let transaction_hash = starknet.add_invoke_transaction_v1(invoke_transaction).unwrap();
        let transaction = starknet.transactions.get_by_hash_mut(&transaction_hash).unwrap();
        assert_eq!(transaction.status, TransactionStatus::Rejected);

        match transaction.execution_error.as_ref().unwrap() {
            starknet_in_rust::transaction::error::TransactionError::InvalidTransactionNonce(
                _,
                _,
            ) => {}
            err => panic!("Invalid error type {:?}", err),
        }
    }

    /// Initialize starknet object with: erc20 contract, account contract and  simple contract that
    /// has a function increase_balance
    fn setup() -> (Starknet, ContractAddress, ContractAddress, Felt, ContractStorageKey) {
        let mut starknet = Starknet::default();

        // deploy erc20 contract
        let erc_20_contract = predeployed::create_erc20().unwrap();
        erc_20_contract.deploy(&mut starknet.state).unwrap();

        // deploy account contract
        let account_without_validations_contract_class = cairo_0_account_without_validations();
        let account_without_validations_class_hash =
            account_without_validations_contract_class.generate_hash().unwrap();

        let account = Account::new(
            Felt::from(10000),
            dummy_felt(),
            dummy_felt(),
            account_without_validations_class_hash,
            ContractClass::Cairo0(account_without_validations_contract_class),
            erc_20_contract.get_address(),
        )
        .unwrap();

        account.deploy(&mut starknet.state).unwrap();
        account.set_initial_balance(&mut starknet.state).unwrap();

        // dummy contract
        let dummy_contract: Cairo0ContractClass = dummy_cairo_0_contract_class().into();
        let sir = StarknetInRustContractClass::try_from(dummy_contract.clone()).unwrap();
        let increase_balance_selector = get_selector_from_name("increase_balance").unwrap();

        // check if increase_balance function is present in the contract class
        sir.entry_points_by_type()
            .get(&EntryPointType::External)
            .unwrap()
            .iter()
            .find(|el| el.selector().to_be_bytes() == increase_balance_selector.to_bytes_be())
            .unwrap();

        let mut address_bytes = get_bytes_from_u32(5);
        address_bytes.reverse();

        let dummy_contract_address =
            ContractAddress::new(Felt::new(address_bytes).unwrap()).unwrap();
        let dummy_contract_class_hash = dummy_contract.generate_hash().unwrap();
        let storage_key = get_storage_var_address("balance", &[]).unwrap();
        let contract_storage_key = ContractStorageKey::new(dummy_contract_address, storage_key);

        // declare dummy contract
        starknet
            .state
            .declare_contract_class(dummy_contract_class_hash, dummy_contract.into())
            .unwrap();

        // deploy dummy contract
        starknet.state.deploy_contract(dummy_contract_address, dummy_contract_class_hash).unwrap();
        // change storage of dummy contract
        // starknet.state.change_storage(contract_storage_key, Felt::from(0)).unwrap();

        starknet.state.synchronize_states();
        starknet.block_context = Starknet::get_block_context(
            1,
            constants::ERC20_CONTRACT_ADDRESS,
            DEVNET_DEFAULT_CHAIN_ID,
        )
        .unwrap();

        starknet.restart_pending_block().unwrap();

        (
            starknet,
            account.get_address(),
            dummy_contract_address,
            Felt::from(increase_balance_selector),
            contract_storage_key,
        )
    }
}
