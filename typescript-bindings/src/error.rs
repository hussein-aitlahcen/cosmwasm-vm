use alloc::string::{String, ToString};
use core::fmt::Display;
use cosmwasm_vm::{
    executor::ExecutorError,
    memory::{MemoryReadError, MemoryWriteError},
    system::SystemError,
};
use cosmwasm_vm_wasmi::WasmiVMError;
use wasmi::CanResume;

#[derive(Debug)]
pub enum SimpleVMError {
    Interpreter(wasmi::Error),
    VMError(WasmiVMError),
    InvalidAccountFormat,
    Frontend(String),
}
impl From<String> for SimpleVMError {
    fn from(error: String) -> Self {
        SimpleVMError::Frontend(error)
    }
}
impl From<&str> for SimpleVMError {
    fn from(error: &str) -> Self {
        error.to_string().into()
    }
}
impl From<wasmi::Error> for SimpleVMError {
    fn from(e: wasmi::Error) -> Self {
        Self::Interpreter(e)
    }
}
impl From<WasmiVMError> for SimpleVMError {
    fn from(e: WasmiVMError) -> Self {
        SimpleVMError::VMError(e)
    }
}
impl From<SystemError> for SimpleVMError {
    fn from(e: SystemError) -> Self {
        SimpleVMError::VMError(e.into())
    }
}
impl From<ExecutorError> for SimpleVMError {
    fn from(e: ExecutorError) -> Self {
        SimpleVMError::VMError(e.into())
    }
}
impl From<MemoryReadError> for SimpleVMError {
    fn from(e: MemoryReadError) -> Self {
        SimpleVMError::VMError(e.into())
    }
}
impl From<MemoryWriteError> for SimpleVMError {
    fn from(e: MemoryWriteError) -> Self {
        SimpleVMError::VMError(e.into())
    }
}
impl Display for SimpleVMError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl CanResume for SimpleVMError {
    fn can_resume(&self) -> bool {
        false
    }
}
