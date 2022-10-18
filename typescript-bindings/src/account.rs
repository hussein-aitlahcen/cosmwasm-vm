use crate::error::SimpleVMError;
use alloc::format;
use alloc::string::{String, ToString};
use core::str::FromStr;
use cosmwasm_minimal_std::Addr;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{convert::FromWasmAbi, describe::WasmDescribe};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BankAccount(pub String);

impl WasmDescribe for BankAccount {
    fn describe() {
        <u32 as WasmDescribe>::describe()
    }
}

impl FromWasmAbi for BankAccount {
    type Abi = <String as FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        BankAccount(String::from_abi(js))
    }
}

impl FromStr for BankAccount {
    type Err = SimpleVMError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BankAccount::try_from(String::from(s))
    }
}

impl TryFrom<Addr> for BankAccount {
    type Error = SimpleVMError;
    fn try_from(value: Addr) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl TryFrom<String> for BankAccount {
    type Error = SimpleVMError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(BankAccount(value))
    }
}

impl From<BankAccount> for Addr {
    fn from(BankAccount(account): BankAccount) -> Self {
        Addr::unchecked(format!("{}", account))
    }
}
