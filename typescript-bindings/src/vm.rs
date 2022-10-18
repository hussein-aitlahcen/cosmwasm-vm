use crate::account::BankAccount;
use crate::error::SimpleVMError;
use crate::marshall::{FatPtr, Marshall};
use crate::typescript::TypeScriptCosmWasmVM;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::{string::String, vec::Vec};
use core::fmt::Display;
use core::num::NonZeroU32;
use cosmwasm_minimal_std::{
    Binary, CanonicalAddr, Coin, CosmwasmQueryResult, Empty, Env, Event, MessageInfo, Order,
    QueryRequest, QueryResult, SystemResult,
};
use cosmwasm_vm::executor::{ExecuteInput, InstantiateInput, MigrateInput};
use cosmwasm_vm::system::{
    cosmwasm_system_entrypoint, cosmwasm_system_query, cosmwasm_system_run, CosmwasmCallVM,
};
use cosmwasm_vm::{
    has::Has,
    memory::{Pointable, ReadWriteMemory, ReadableMemory, WritableMemory},
    system::CosmwasmContractMeta,
    transaction::Transactional,
    vm::{VMBase, VmErrorOf, VmGas, VmGasCheckpoint},
};
use cosmwasm_vm_wasmi::{
    new_wasmi_vm, WasmiHostFunction, WasmiHostFunctionIndex, WasmiImportResolver, WasmiInput,
    WasmiModule, WasmiModuleExecutor, WasmiOutput, WasmiVM, WasmiVMError,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_instrument::gas_metering::Rules;

pub struct SimpleWasmiVM<'a> {
    pub host_functions: BTreeMap<WasmiHostFunctionIndex, WasmiHostFunction<Self>>,
    pub executing_module: WasmiModule,
    pub typescript_vm: &'a TypeScriptCosmWasmVM,
}

impl<'a> WasmiModuleExecutor for SimpleWasmiVM<'a> {
    fn executing_module(&self) -> WasmiModule {
        self.executing_module.clone()
    }
    fn host_function(&self, index: WasmiHostFunctionIndex) -> Option<&WasmiHostFunction<Self>> {
        self.host_functions.get(&index)
    }
}

impl<'a> Pointable for SimpleWasmiVM<'a> {
    type Pointer = u32;
}

impl<'a> ReadableMemory for SimpleWasmiVM<'a> {
    type Error = VmErrorOf<Self>;
    fn read(&self, offset: Self::Pointer, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.executing_module
            .memory
            .get_into(offset, buffer)
            .map_err(|_| WasmiVMError::LowLevelMemoryReadError.into())
    }
}

impl<'a> WritableMemory for SimpleWasmiVM<'a> {
    type Error = VmErrorOf<Self>;
    fn write(&self, offset: Self::Pointer, buffer: &[u8]) -> Result<(), Self::Error> {
        self.executing_module
            .memory
            .set(offset, buffer)
            .map_err(|_| WasmiVMError::LowLevelMemoryWriteError.into())
    }
}

impl<'a> ReadWriteMemory for SimpleWasmiVM<'a> {}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct CanonicalAddress(pub CanonicalAddr);

impl TryFrom<Vec<u8>> for CanonicalAddress {
    type Error = SimpleVMError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(CanonicalAddress(CanonicalAddr(Binary::from(value))))
    }
}

impl From<CanonicalAddress> for Vec<u8> {
    fn from(addr: CanonicalAddress) -> Self {
        addr.0.into()
    }
}

impl From<CanonicalAddress> for CanonicalAddr {
    fn from(addr: CanonicalAddress) -> Self {
        addr.0
    }
}

impl<'a> VMBase for SimpleWasmiVM<'a> {
    type Input<'x> = WasmiInput<'x, WasmiVM<Self>>;
    type Output<'x> = WasmiOutput<'x, WasmiVM<Self>>;
    type QueryCustom = Empty;
    type MessageCustom = Empty;
    type ContractMeta = CosmwasmContractMeta<BankAccount>;
    type Address = BankAccount;
    type CanonicalAddress = CanonicalAddress;
    type StorageKey = Vec<u8>;
    type StorageValue = Vec<u8>;
    type Error = SimpleVMError;

    fn running_contract_meta(&mut self) -> Result<Self::ContractMeta, Self::Error> {
        self.typescript_vm
            .running_contract_meta()
            .0
            .map_err(From::from)
    }

    fn set_contract_meta(
        &mut self,
        address: Self::Address,
        contract_meta: Self::ContractMeta,
    ) -> Result<(), Self::Error> {
        self.typescript_vm
            .set_contract_meta(address.into(), contract_meta.into())
            .0
            .map_err(From::from)
    }

    fn contract_meta(&mut self, address: Self::Address) -> Result<Self::ContractMeta, Self::Error> {
        self.typescript_vm
            .contract_meta(address.into())
            .0
            .map_err(From::from)
    }

    fn query_continuation(
        &mut self,
        address: Self::Address,
        message: &[u8],
    ) -> Result<QueryResult, Self::Error> {
        self.typescript_vm
            .query_continuation(address.into(), message)
            .0
            .map_err(From::from)
    }

    fn continue_execute(
        &mut self,
        address: Self::Address,
        funds: Vec<Coin>,
        message: &[u8],
        event_handler: &mut dyn FnMut(Event),
    ) -> Result<Option<Binary>, Self::Error> {
        let handler_ptr = unsafe { core::mem::transmute(event_handler) };
        self.typescript_vm
            .continue_execute(address.into(), funds.into(), message, handler_ptr)
            .0
            .map_err(From::from)
    }

    fn continue_instantiate(
        &mut self,
        contract_meta: Self::ContractMeta,
        funds: Vec<Coin>,
        message: &[u8],
        event_handler: &mut dyn FnMut(Event),
    ) -> Result<(Self::Address, Option<Binary>), Self::Error> {
        let handler_ptr = unsafe { core::mem::transmute(event_handler) };
        self.typescript_vm
            .continue_instantiate(contract_meta.into(), funds.into(), message, handler_ptr)
            .0
            .map_err(From::from)
    }

    fn continue_migrate(
        &mut self,
        address: Self::Address,
        message: &[u8],
        event_handler: &mut dyn FnMut(Event),
    ) -> Result<Option<Binary>, Self::Error> {
        let handler_ptr = unsafe { core::mem::transmute(event_handler) };
        self.typescript_vm
            .continue_migrate(address.into(), message, handler_ptr)
            .0
            .map_err(From::from)
    }

    fn query_custom(
        &mut self,
        _: Self::QueryCustom,
    ) -> Result<SystemResult<CosmwasmQueryResult>, Self::Error> {
        Err("Query Custom is not supported".into())
    }

    fn message_custom(
        &mut self,
        _: Self::MessageCustom,
        _: &mut dyn FnMut(Event),
    ) -> Result<Option<Binary>, Self::Error> {
        Err("Message Custom is not supported".into())
    }

    fn query_raw(
        &mut self,
        address: Self::Address,
        key: Self::StorageKey,
    ) -> Result<Option<Self::StorageValue>, Self::Error> {
        self.typescript_vm
            .query_raw(address.into(), key)
            .0
            .map_err(From::from)
    }

    fn transfer(&mut self, to: &Self::Address, funds: &[Coin]) -> Result<(), Self::Error> {
        self.typescript_vm
            .transfer(to.clone().into(), funds.to_vec().into())
            .0
            .map_err(From::from)
    }

    fn burn(&mut self, funds: &[Coin]) -> Result<(), Self::Error> {
        self.typescript_vm
            .burn(funds.to_vec().into())
            .0
            .map_err(From::from)
    }

    fn balance(&mut self, address: &Self::Address, denom: String) -> Result<Coin, Self::Error> {
        self.typescript_vm
            .balance(address.clone().into(), denom)
            .0
            .map_err(From::from)
    }

    fn all_balance(&mut self, address: &Self::Address) -> Result<Vec<Coin>, Self::Error> {
        self.typescript_vm
            .all_balance(address.clone().into())
            .0
            .map_err(From::from)
    }

    fn query_info(
        &mut self,
        address: Self::Address,
    ) -> Result<cosmwasm_minimal_std::ContractInfoResponse, Self::Error> {
        self.typescript_vm
            .query_info(address.clone().into())
            .0
            .map_err(From::from)
    }

    fn debug(&mut self, message: Vec<u8>) -> Result<(), Self::Error> {
        self.typescript_vm.debug(message).0.map_err(From::from)
    }

    fn db_scan(
        &mut self,
        start: Option<Self::StorageKey>,
        end: Option<Self::StorageKey>,
        order: Order,
    ) -> Result<u32, Self::Error> {
        self.typescript_vm
            .db_scan(start.into(), end.into(), order.into())
            .0
            .map_err(From::from)
    }

    fn db_next(
        &mut self,
        iterator_id: u32,
    ) -> Result<(Self::StorageKey, Self::StorageValue), Self::Error> {
        self.typescript_vm
            .db_next(iterator_id)
            .0
            .map_err(From::from)
    }

    fn secp256k1_verify(
        &mut self,
        message_hash: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, Self::Error> {
        self.typescript_vm
            .secp256k1_verify(message_hash, signature, public_key)
            .0
            .map_err(From::from)
    }

    fn secp256k1_recover_pubkey(
        &mut self,
        message_hash: &[u8],
        signature: &[u8],
        recovery_param: u8,
    ) -> Result<Result<Vec<u8>, ()>, Self::Error> {
        self.typescript_vm
            .secp256k1_recover_pubkey(message_hash, signature, recovery_param)
            .0
            .map_err(From::from)
    }

    fn ed25519_verify(
        &mut self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, Self::Error> {
        self.typescript_vm
            .ed25519_verify(message, signature, public_key)
            .0
            .map_err(From::from)
    }

    fn ed25519_batch_verify(
        &mut self,
        messages: &[&[u8]],
        signatures: &[&[u8]],
        public_keys: &[&[u8]],
    ) -> Result<bool, Self::Error> {
        self.typescript_vm
            .ed25519_batch_verify(
                messages
                    .iter()
                    .map(|x| x.to_vec())
                    .collect::<Vec<_>>()
                    .into(),
                signatures
                    .iter()
                    .map(|x| x.to_vec())
                    .collect::<Vec<_>>()
                    .into(),
                public_keys
                    .iter()
                    .map(|x| x.to_vec())
                    .collect::<Vec<_>>()
                    .into(),
            )
            .0
            .map_err(From::from)
    }

    fn addr_validate(&mut self, input: &str) -> Result<Result<(), Self::Error>, Self::Error> {
        self.typescript_vm
            .addr_validate(input)
            .0
            .map(|e| e.map_err(From::from))
            .map_err(From::from)
    }

    fn addr_canonicalize(
        &mut self,
        input: &str,
    ) -> Result<Result<Self::CanonicalAddress, Self::Error>, Self::Error> {
        self.typescript_vm
            .addr_canonicalize(input)
            .0
            .map(|e| e.map_err(From::from))
            .map_err(From::from)
    }

    fn addr_humanize(
        &mut self,
        addr: &Self::CanonicalAddress,
    ) -> Result<Result<Self::Address, Self::Error>, Self::Error> {
        self.typescript_vm
            .addr_humanize(addr.clone().into())
            .0
            .map(|e| e.map_err(From::from))
            .map_err(From::from)
    }

    fn db_read(
        &mut self,
        key: Self::StorageKey,
    ) -> Result<Option<Self::StorageValue>, Self::Error> {
        self.typescript_vm.db_read(key).0.map_err(From::from)
    }

    fn db_write(
        &mut self,
        key: Self::StorageKey,
        value: Self::StorageValue,
    ) -> Result<(), Self::Error> {
        self.typescript_vm
            .db_write(key, value)
            .0
            .map_err(From::from)
    }

    fn db_remove(&mut self, key: Self::StorageKey) -> Result<(), Self::Error> {
        self.typescript_vm.db_remove(key).0.map_err(From::from)
    }

    fn abort(&mut self, message: String) -> Result<(), Self::Error> {
        self.typescript_vm.abort(message).0.map_err(From::from)
    }

    fn charge(&mut self, value: VmGas) -> Result<(), Self::Error> {
        self.typescript_vm
            .charge(value.into())
            .0
            .map_err(From::from)
    }

    fn gas_checkpoint_push(&mut self, checkpoint: VmGasCheckpoint) -> Result<(), Self::Error> {
        self.typescript_vm
            .gas_checkpoint_push(checkpoint.into())
            .0
            .map_err(From::from)
    }

    fn gas_checkpoint_pop(&mut self) -> Result<(), Self::Error> {
        self.typescript_vm
            .gas_checkpoint_pop()
            .0
            .map_err(From::from)
    }

    fn gas_ensure_available(&mut self) -> Result<(), Self::Error> {
        self.typescript_vm
            .gas_ensure_available()
            .0
            .map_err(From::from)
    }
}

impl<'a> Has<Env> for SimpleWasmiVM<'a> {
    fn get(&self) -> Env {
        self.typescript_vm.env().0
    }
}
impl<'a> Has<MessageInfo> for SimpleWasmiVM<'a> {
    fn get(&self) -> MessageInfo {
        self.typescript_vm.info().0
    }
}

impl<'a> Transactional for SimpleWasmiVM<'a> {
    type Error = SimpleVMError;
    fn transaction_begin(&mut self) -> Result<(), Self::Error> {
        self.typescript_vm.transaction_begin().0.map_err(From::from)
    }
    fn transaction_commit(&mut self) -> Result<(), Self::Error> {
        self.typescript_vm
            .transaction_commit()
            .0
            .map_err(From::from)
    }
    fn transaction_rollback(&mut self) -> Result<(), Self::Error> {
        self.typescript_vm
            .transaction_rollback()
            .0
            .map_err(From::from)
    }
}

struct ConstantCostRules;
impl Rules for ConstantCostRules {
    fn instruction_cost(
        &self,
        _: &wasm_instrument::parity_wasm::elements::Instruction,
    ) -> Option<u32> {
        Some(1)
    }

    fn memory_grow_cost(&self) -> wasm_instrument::gas_metering::MemoryGrowCost {
        wasm_instrument::gas_metering::MemoryGrowCost::Linear(
            NonZeroU32::new(1024).expect("impossible"),
        )
    }
}

pub fn vm_create<'a>(
    typescript_vm: &'a TypeScriptCosmWasmVM,
    code: &[u8],
) -> WasmiVM<SimpleWasmiVM<'a>> {
    let host_functions_definitions =
        WasmiImportResolver(cosmwasm_vm_wasmi::host_functions::definitions());
    let module = new_wasmi_vm(&host_functions_definitions, code).unwrap();
    WasmiVM(SimpleWasmiVM {
        host_functions: host_functions_definitions
            .0
            .clone()
            .into_iter()
            .flat_map(|(_, modules)| modules.into_iter().map(|(_, function)| function))
            .collect(),
        executing_module: module,
        typescript_vm,
    })
}

pub fn vm_call<'a, I>(
    typescript_vm: &'a TypeScriptCosmWasmVM,
    code: &[u8],
    message: String,
) -> Marshall<Result<VMStep, String>>
where
    WasmiVM<SimpleWasmiVM<'a>>: CosmwasmCallVM<I>,
    VmErrorOf<WasmiVM<SimpleWasmiVM<'a>>>: Display,
{
    let mut vm = vm_create(typescript_vm, code);
    let result =
        cosmwasm_system_entrypoint::<I, WasmiVM<SimpleWasmiVM>>(&mut vm, message.as_bytes());
    Marshall(match result {
        Ok((data, events)) => Ok(VMStep { events, data }),
        Err(e) => Err(format!("{}", e)),
    })
}

pub fn vm_continue<'a, I>(
    typescript_vm: &'a TypeScriptCosmWasmVM,
    code: &[u8],
    message: String,
    event_handler: FatPtr,
) -> Marshall<Result<Option<Binary>, String>>
where
    WasmiVM<SimpleWasmiVM<'a>>: CosmwasmCallVM<I>,
    VmErrorOf<WasmiVM<SimpleWasmiVM<'a>>>: Display,
{
    let mut vm = vm_create(typescript_vm, code);
    cosmwasm_system_run::<I, WasmiVM<SimpleWasmiVM>>(&mut vm, message.as_bytes(), unsafe {
        core::mem::transmute(event_handler)
    })
    .map_err(|e| format!("{}", e))
    .into()
}

#[derive(Serialize, Deserialize)]
pub struct VMStep {
    events: Vec<Event>,
    data: Option<Binary>,
}

#[wasm_bindgen]
pub fn vm_instantiate(
    typescript_vm: &TypeScriptCosmWasmVM,
    code: &[u8],
    message: String,
) -> Marshall<Result<VMStep, String>> {
    vm_call::<InstantiateInput>(typescript_vm, code, message)
}

#[wasm_bindgen]
pub fn vm_execute(
    typescript_vm: &TypeScriptCosmWasmVM,
    code: &[u8],
    message: String,
) -> Marshall<Result<VMStep, String>> {
    vm_call::<ExecuteInput>(typescript_vm, code, message)
}

#[wasm_bindgen]
pub fn vm_migrate(
    typescript_vm: &TypeScriptCosmWasmVM,
    code: &[u8],
    message: String,
) -> Marshall<Result<VMStep, String>> {
    vm_call::<MigrateInput>(typescript_vm, code, message)
}

#[wasm_bindgen]
pub fn vm_continue_instantiate(
    typescript_vm: &TypeScriptCosmWasmVM,
    code: &[u8],
    message: String,
    event_handler: FatPtr,
) -> Marshall<Result<Option<Binary>, String>> {
    vm_continue::<InstantiateInput>(typescript_vm, code, message, event_handler)
}

#[wasm_bindgen]
pub fn vm_continue_execute(
    typescript_vm: &TypeScriptCosmWasmVM,
    code: &[u8],
    message: String,
    event_handler: FatPtr,
) -> Marshall<Result<Option<Binary>, String>> {
    vm_continue::<ExecuteInput>(typescript_vm, code, message, event_handler)
}

#[wasm_bindgen]
pub fn vm_continue_migrate(
    typescript_vm: &TypeScriptCosmWasmVM,
    code: &[u8],
    message: String,
    event_handler: FatPtr,
) -> Marshall<Result<Option<Binary>, String>> {
    vm_continue::<MigrateInput>(typescript_vm, code, message, event_handler)
}

#[wasm_bindgen]
pub fn vm_query(
    typescript_vm: &TypeScriptCosmWasmVM,
    code: &[u8],
    Marshall(query): Marshall<QueryRequest>,
) -> Marshall<Result<Result<CosmwasmQueryResult, String>, String>> {
    let mut vm = vm_create(typescript_vm, code);
    let result = cosmwasm_system_query(&mut vm, query);
    Marshall(
        result
            .map(|x| x.into_result().map_err(|e| format!("{:?}", e)))
            .map_err(|e| format!("{}", e)),
    )
}
