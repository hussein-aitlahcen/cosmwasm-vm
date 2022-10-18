use alloc::format;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi},
    describe::WasmDescribe,
    prelude::wasm_bindgen,
    JsValue,
};

#[cfg(target_pointer_width = "32")]
pub type FatPtr = u64;
#[cfg(target_pointer_width = "64")]
pub type FatPtr = Marshall<u128>;

pub struct Marshall<T>(pub T);

impl<T> From<T> for Marshall<T> {
    fn from(x: T) -> Self {
        Marshall(x)
    }
}

impl<T> WasmDescribe for Marshall<T> {
    fn describe() {
        JsValue::describe()
    }
}

impl<T> FromWasmAbi for Marshall<T>
where
    T: DeserializeOwned,
{
    type Abi = <JsValue as FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        let value = JsValue::from_abi(js);
        Marshall(
            serde_wasm_bindgen::from_value::<T>(value)
                .map_err(|e| log(&format!("{:?}", e)))
                .unwrap(),
        )
    }
}

impl<T> IntoWasmAbi for Marshall<T>
where
    T: Serialize,
{
    type Abi = <JsValue as IntoWasmAbi>::Abi;
    fn into_abi(self) -> Self::Abi {
        JsValue::into_abi(
            serde_wasm_bindgen::to_value(&self.0)
                .map_err(|e| log(&format!("{:?}", e)))
                .unwrap(),
        )
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
