use crate::{
    account::BankAccount,
    marshall::{FatPtr, Marshall},
    vm::CanonicalAddress,
};
use alloc::{string::String, vec::Vec};
use cosmwasm_minimal_std::{Binary, Coin, Env, MessageInfo, Order, QueryResult};
use cosmwasm_vm::{
    system::CosmwasmContractMeta,
    vm::{VmGas, VmGasCheckpoint},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub type TypeScriptCosmWasmVM;

    #[wasm_bindgen(structural, method)]
    pub fn env(this: &TypeScriptCosmWasmVM) -> Marshall<Env>;

    #[wasm_bindgen(structural, method)]
    pub fn info(this: &TypeScriptCosmWasmVM) -> Marshall<MessageInfo>;

    #[wasm_bindgen(structural, method)]
    pub fn running_contract_meta(
        this: &TypeScriptCosmWasmVM,
    ) -> Marshall<Result<CosmwasmContractMeta<BankAccount>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn set_contract_meta(
        this: &TypeScriptCosmWasmVM,
        address: Marshall<BankAccount>,
        contract_meta: Marshall<CosmwasmContractMeta<BankAccount>>,
    ) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn contract_meta(
        this: &TypeScriptCosmWasmVM,
        address: Marshall<BankAccount>,
    ) -> Marshall<Result<CosmwasmContractMeta<BankAccount>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn query_continuation(
        this: &TypeScriptCosmWasmVM,
        address: Marshall<BankAccount>,
        message: &[u8],
    ) -> Marshall<Result<QueryResult, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn continue_execute(
        this: &TypeScriptCosmWasmVM,
        address: Marshall<BankAccount>,
        funds: Marshall<Vec<Coin>>,
        message: &[u8],
        event_handler: FatPtr,
    ) -> Marshall<Result<Option<Binary>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn continue_instantiate(
        this: &TypeScriptCosmWasmVM,
        contract_meta: Marshall<CosmwasmContractMeta<BankAccount>>,
        funds: Marshall<Vec<Coin>>,
        message: &[u8],
        event_handler: FatPtr,
    ) -> Marshall<Result<(BankAccount, Option<Binary>), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn continue_migrate(
        this: &TypeScriptCosmWasmVM,
        address: Marshall<BankAccount>,
        message: &[u8],
        event_handler: FatPtr,
    ) -> Marshall<Result<Option<Binary>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn query_raw(
        this: &TypeScriptCosmWasmVM,
        address: Marshall<BankAccount>,
        key: Vec<u8>,
    ) -> Marshall<Result<Option<Vec<u8>>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn transfer(
        this: &TypeScriptCosmWasmVM,
        to: Marshall<BankAccount>,
        funds: Marshall<Vec<Coin>>,
    ) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn burn(
        this: &TypeScriptCosmWasmVM,
        funds: Marshall<Vec<Coin>>,
    ) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn balance(
        this: &TypeScriptCosmWasmVM,
        address: Marshall<BankAccount>,
        denom: String,
    ) -> Marshall<Result<Coin, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn all_balance(
        this: &TypeScriptCosmWasmVM,
        address: Marshall<BankAccount>,
    ) -> Marshall<Result<Vec<Coin>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn query_info(
        this: &TypeScriptCosmWasmVM,
        address: Marshall<BankAccount>,
    ) -> Marshall<Result<cosmwasm_minimal_std::ContractInfoResponse, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn debug(this: &TypeScriptCosmWasmVM, message: Vec<u8>) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn db_scan(
        this: &TypeScriptCosmWasmVM,
        start: Option<Vec<u8>>,
        end: Option<Vec<u8>>,
        order: Marshall<Order>,
    ) -> Marshall<Result<u32, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn db_next(
        this: &TypeScriptCosmWasmVM,
        iterator_id: u32,
    ) -> Marshall<Result<(Vec<u8>, Vec<u8>), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn secp256k1_verify(
        this: &TypeScriptCosmWasmVM,
        message_hash: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Marshall<Result<bool, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn secp256k1_recover_pubkey(
        this: &TypeScriptCosmWasmVM,
        message_hash: &[u8],
        signature: &[u8],
        recovery_param: u8,
    ) -> Marshall<Result<Result<Vec<u8>, ()>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn ed25519_verify(
        this: &TypeScriptCosmWasmVM,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Marshall<Result<bool, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn ed25519_batch_verify(
        this: &TypeScriptCosmWasmVM,
        messages: Marshall<Vec<Vec<u8>>>,
        signatures: Marshall<Vec<Vec<u8>>>,
        public_keys: Marshall<Vec<Vec<u8>>>,
    ) -> Marshall<Result<bool, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn addr_validate(
        this: &TypeScriptCosmWasmVM,
        input: &str,
    ) -> Marshall<Result<Result<(), String>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn addr_canonicalize(
        this: &TypeScriptCosmWasmVM,
        input: &str,
    ) -> Marshall<Result<Result<CanonicalAddress, String>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn addr_humanize(
        this: &TypeScriptCosmWasmVM,
        addr: Marshall<CanonicalAddress>,
    ) -> Marshall<Result<Result<BankAccount, String>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn db_read(
        this: &TypeScriptCosmWasmVM,
        key: Vec<u8>,
    ) -> Marshall<Result<Option<Vec<u8>>, String>>;

    #[wasm_bindgen(structural, method)]
    pub fn db_write(
        this: &TypeScriptCosmWasmVM,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn db_remove(this: &TypeScriptCosmWasmVM, key: Vec<u8>) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn abort(this: &TypeScriptCosmWasmVM, message: String) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn charge(
        this: &TypeScriptCosmWasmVM,
        value: Marshall<VmGas>,
    ) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn gas_checkpoint_push(
        this: &TypeScriptCosmWasmVM,
        checkpoint: Marshall<VmGasCheckpoint>,
    ) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn gas_checkpoint_pop(this: &TypeScriptCosmWasmVM) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn gas_ensure_available(this: &TypeScriptCosmWasmVM) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn transaction_begin(this: &TypeScriptCosmWasmVM) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn transaction_commit(this: &TypeScriptCosmWasmVM) -> Marshall<Result<(), String>>;

    #[wasm_bindgen(structural, method)]
    pub fn transaction_rollback(this: &TypeScriptCosmWasmVM) -> Marshall<Result<(), String>>;
}
