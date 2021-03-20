#![allow(dead_code)]
use super::*;
use primitive_types::{H160, H256, U256};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

pub type RawAddress = [u8; 20];
pub type RawU256 = [u8; 32];
pub type RawH256 = [u8; 32];
pub type AccountId = String;
pub type Balance = u128;
pub type Gas = u64;
pub type StorageUsage = u64;

#[derive(BorshSerialize)]
pub struct StorageBalance {
    pub total: u128,
    pub available: u128,
}

#[derive(Serialize)]
pub struct StorageBalanceBounds {
    pub min: u128,
    pub max: Option<u128>,
}

#[derive(Clone)]
pub struct Log {
    pub address: H160,
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct InitCallArgs {
    pub prover_account: AccountId,
    pub eth_custodian_address: AccountId,
}

#[derive(Debug, Deserialize)]
pub struct BalanceOfCallArgs {
    pub account_id: AccountId,
}

#[derive(Debug, Deserialize)]
pub struct TransferCallArgs {
    pub receiver_id: AccountId,
    pub amount: Balance,
    pub memo: Option<String>,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct EthConnector {
    pub prover_account: AccountId,
    pub eth_custodian_address: EthAddress,
    pub used_events: BTreeSet<Vec<u8>>,
    pub token: FungibleToken,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FinishDepositCallArgs {
    pub new_owner_id: AccountId,
    pub amount: u128,
    pub fee: u128,
    pub proof: Proof,
}

#[derive(Deserialize)]
pub struct WithdrawCallArgs {
    pub recipient_id: AccountId,
    pub amount: u128,
}

#[derive(Deserialize)]
pub struct TransferCallCallArgs {
    pub receiver_id: AccountId,
    pub amount: Balance,
    pub memo: Option<String>,
    pub msg: String,
}

#[derive(Deserialize)]
pub struct ResolveTransferCallArgs {
    pub sender_id: AccountId,
    pub receiver_id: AccountId,
    pub amount: Balance,
}

#[derive(Deserialize)]
pub struct StorageBalanceOfCallArgs {
    pub account_id: AccountId,
}

#[derive(Deserialize)]
pub struct StorageWithdrawCallArgs {
    pub amount: Option<u128>,
}

#[derive(Deserialize)]
pub struct StorageDepositCallArgs {
    pub account_id: Option<AccountId>,
    pub registration_only: Option<bool>,
}

#[derive(BorshSerialize)]
pub struct WithdrawResult {
    pub amount: Balance,
    pub recipient_id: RawAddress,
    pub eth_custodian_address: RawAddress,
}

#[derive(BorshSerialize)]
pub struct FtOnTransfer {
    pub amount: Balance,
    pub msg: String,
    pub receiver_id: AccountId,
}

#[derive(BorshSerialize)]
pub struct FtResolveTransfer {
    pub receiver_id: AccountId,
    pub amount: Balance,
    pub current_account_id: AccountId,
}

pub enum PromiseResult {
    NotReady,
    Successful(Vec<u8>),
    Failed,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FunctionCallArgs {
    pub contract: RawAddress,
    pub input: Vec<u8>,
}

#[derive(Serialize)]
pub struct FtResolveTransferResult {
    pub amount: Balance,
    pub refund_amount: Balance,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ViewCallArgs {
    pub sender: RawAddress,
    pub address: RawAddress,
    pub amount: RawU256,
    pub input: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GetStorageAtArgs {
    pub address: RawAddress,
    pub key: RawH256,
}

pub enum KeyPrefix {
    Code = 0x0,
    Balance = 0x1,
    Nonce = 0x2,
    Storage = 0x3,
}

pub fn address_to_key(prefix: KeyPrefix, address: &H160) -> [u8; 21] {
    let mut result = [0u8; 21];
    result[0] = prefix as u8;
    result[1..].copy_from_slice(&address.0);
    result
}

pub fn storage_to_key(address: &H160, key: &H256) -> [u8; 53] {
    let mut result = [0u8; 53];
    result[0] = KeyPrefix::Storage as u8;
    result[1..21].copy_from_slice(&address.0);
    result[21..].copy_from_slice(&key.0);
    result
}

pub fn u256_to_arr(value: &U256) -> [u8; 32] {
    let mut result = [0u8; 32];
    value.to_big_endian(&mut result);
    result
}

pub fn log_to_bytes(log: Log) -> Vec<u8> {
    let mut result = vec![0u8; 1 + log.topics.len() * 32 + log.data.len()];
    result[0] = log.topics.len() as u8;
    let mut index = 1;
    for topic in log.topics.iter() {
        result[index..index + 32].copy_from_slice(&topic.0);
        index += 32;
    }
    result[index..].copy_from_slice(&log.data);
    result
}

const HEX_ALPHABET: &[u8; 16] = b"0123456789abcdef";

pub fn bytes_to_hex(v: &[u8]) -> String {
    let mut result = String::new();
    for x in v {
        result.push(HEX_ALPHABET[(x / 16) as usize] as char);
        result.push(HEX_ALPHABET[(x % 16) as usize] as char);
    }
    result
}

#[cfg(not(feature = "contract"))]
#[inline]
pub fn keccak(data: &[u8]) -> H256 {
    H256::from_slice(Keccak256::digest(data).as_slice())
}

pub fn near_account_to_evm_address(addr: &[u8]) -> H160 {
    H160::from_slice(&keccak(addr)[12..])
}
