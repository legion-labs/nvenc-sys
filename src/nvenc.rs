#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(clippy::too_many_arguments)]

#[cfg(feature = "use-bindgen")]
include!(concat!(env!("OUT_DIR"), "/nvenc.rs"));

#[cfg(not(feature = "use-bindgen"))]
include!("./bindgen/nvenc.rs");

pub const NV_ENCODE_API_GET_MAX_SUPPORTED_VERSION_FN_NAME: &[u8] =
    b"NvEncodeAPIGetMaxSupportedVersion\0";
pub type NvEncodeApiGetMaxSupportedVersionFn =
    unsafe extern "C" fn(version: *mut u32) -> NVENCSTATUS;

pub const NV_ENCODE_API_CREATE_INSTANCE_FN_NAME: &[u8] = b"NvEncodeAPICreateInstance\0";
pub type NvEncodeApiCreateInstanceFn =
    unsafe extern "C" fn(functionList: *mut NV_ENCODE_API_FUNCTION_LIST) -> NVENCSTATUS;

#[cfg(windows)]
pub const NVENC_DLL_NAME: &str = "nvEncodeAPI64.dll";

#[cfg(not(windows))]
pub const NVENC_DLL_NAME: &str = "libnvidia-encode.so.1";

mod validation {
    use super::*;
    const _NV_ENCODE_API_GET_MAX_SUPPORTED_VERSION: NvEncodeApiGetMaxSupportedVersionFn =
        NvEncodeAPIGetMaxSupportedVersion;
    const _NV_ENCODE_API_CREATE_INSTANCE: NvEncodeApiCreateInstanceFn = NvEncodeAPICreateInstance;
}

#[cfg(test)]
mod tests {
    use crate::{
        NvEncodeApiCreateInstanceFn, NVENCSTATUS, NVENC_DLL_NAME,
        NV_ENCODE_API_CREATE_INSTANCE_FN_NAME, NV_ENCODE_API_FUNCTION_LIST,
        NV_ENCODE_API_FUNCTION_LIST_VER,
    };

    #[test]
    #[ignore = "needs nvidia gpu to test"]
    fn nvenc_tests() {
        let nvenc = unsafe { libloading::Library::new(NVENC_DLL_NAME).unwrap() };
        let nv_encode_api_create_instance_fn = unsafe {
            nvenc
                .get::<NvEncodeApiCreateInstanceFn>(NV_ENCODE_API_CREATE_INSTANCE_FN_NAME)
                .expect("failed to load amf init function")
        };
        let mut nvenc_api_fn_list = NV_ENCODE_API_FUNCTION_LIST {
            version: NV_ENCODE_API_FUNCTION_LIST_VER,
            ..NV_ENCODE_API_FUNCTION_LIST::default()
        };
        let status = unsafe { nv_encode_api_create_instance_fn(&mut nvenc_api_fn_list) };
        assert_eq!(status, NVENCSTATUS::NV_ENC_SUCCESS);
    }
}
