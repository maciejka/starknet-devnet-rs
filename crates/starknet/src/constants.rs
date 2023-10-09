use std::net::{IpAddr, Ipv4Addr};

use starknet_types::chain_id::ChainId;

pub(crate) const CAIRO_0_ACCOUNT_CONTRACT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/accounts_artifacts/OpenZeppelin/0.5.1/Account.cairo/Account.json"
);
pub const CAIRO_0_ACCOUNT_CONTRACT_HASH: &str =
    "0x4d07e40e93398ed3c76981e72dd1fd22557a78ce36c0515f679e27f0bb5bc5f";

// TODO consider naming according to the exact version (if we want to support multiple versions)
pub(crate) const CAIRO_1_ACCOUNT_CONTRACT_SIERRA_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/accounts_artifacts/OpenZeppelin/0.7.0/Account.cairo/Account.sierra.json"
);
pub const CAIRO_1_ACCOUNT_CONTRACT_HASH: &str =
    "0x2b513521d389c0477b3a9a90a1ff4822bcd957a9c8ba0dfc49918b59a19cf8a"; // TODO this is casm hash - which one do we need? currently both artifacts are stored

pub(crate) const ERC20_CONTRACT_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/accounts_artifacts/ERC20_Mintable_OZ_0.2.0.json");

pub(crate) const ERC20_CONTRACT_CLASS_HASH: &str =
    "0x6A22BF63C7BC07EFFA39A25DFBD21523D211DB0100A0AFD054D172B81840EAF";

pub const ERC20_CONTRACT_ADDRESS: &str =
    "0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7";

pub(crate) const UDC_CONTRACT_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/accounts_artifacts/UDC_OZ_0.5.0.json");

pub(crate) const UDC_CONTRACT_CLASS_HASH: &str =
    "0x7B3E05F48F0C69E4A65CE5E076A66271A527AFF2C34CE1083EC6E1526997A69";

pub const UDC_CONTRACT_ADDRESS: &str =
    "0x41A78E741E5AF2FEC34B695679BC6891742439F7AFB8484ECD7766661AD02BF";

pub const DEVNET_DEFAULT_SEED: u32 = 123;
pub const DEVNET_DEFAULT_TEST_SEED: u32 = 123;
pub const DEVNET_DEFAULT_TOTAL_ACCOUNTS: u8 = 10;
pub const DEVNET_DEFAULT_INITIAL_BALANCE: u128 = 1_000_000_000_000_000_000_000;
pub const DEVNET_DEFAULT_GAS_PRICE: u64 = 100_000_000_000;
pub const DEVNET_DEFAULT_HOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
pub const DEVNET_DEFAULT_PORT: u16 = 5050;
pub const DEVNET_DEFAULT_TIMEOUT: u16 = 120;
pub const DEVNET_DEFAULT_CHAIN_ID: ChainId = ChainId::TestNet;

pub const SUPPORTED_TX_VERSION: u32 = 1;

// chargeable account
pub const CHARGEABLE_ACCOUNT_PUBLIC_KEY: &str =
    "0x4C37AB4F0994879337BFD4EAD0800776DB57DA382B8ED8EFAA478C5D3B942A4";
pub const CHARGEABLE_ACCOUNT_PRIVATE_KEY: &str = "0x5FB2959E3011A873A7160F5BB32B0ECE";
pub const CHARGEABLE_ACCOUNT_ADDRESS: &str =
    "0x1CAF2DF5ED5DDE1AE3FAEF4ACD72522AC3CB16E23F6DC4C7F9FAED67124C511";
