#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(clippy::too_many_arguments)]

#[cfg(feature = "use-bindgen")]
include!(concat!(env!("OUT_DIR"), "/cuda.rs"));

#[cfg(not(feature = "use-bindgen"))]
include!("./bindgen/cuda.rs");

pub const CU_GET_ERROR_STRING_FN_NAME: &[u8] = b"cuGetErrorString\0";
pub type CuGetErrorStringFn =
    unsafe extern "C" fn(error: CUresult, pStr: *mut *const ::std::os::raw::c_char) -> CUresult;

pub const CU_GET_ERROR_NAME_FN_NAME: &[u8] = b"cuGetErrorName\0";
pub type CuGetErrorNameFn =
    unsafe extern "C" fn(error: CUresult, pStr: *mut *const ::std::os::raw::c_char) -> CUresult;

pub const CU_INIT_FN_NAME: &[u8] = b"cuInit\0";
pub type CuInitFn = unsafe extern "C" fn(flags: ::std::os::raw::c_uint) -> CUresult;

pub const CU_DEVICE_GET_COUNT_FN_NAME: &[u8] = b"cuDeviceGetCount\0";
pub type CuDeviceGetCountFn = unsafe extern "C" fn(count: *mut ::std::os::raw::c_int) -> CUresult;

pub const CU_DEVICE_GET_FN_NAME: &[u8] = b"cuDeviceGet\0";
pub type CuDeviceGetFn =
    unsafe extern "C" fn(device: *mut CUdevice, ordinal: ::std::os::raw::c_int) -> CUresult;

pub const CU_DEVICE_GET_NAME_FN_NAME: &[u8] = b"cuDeviceGetName\0";
pub type CuDeviceGetNameFn = unsafe extern "C" fn(
    name: *mut ::std::os::raw::c_char,
    len: ::std::os::raw::c_int,
    dev: CUdevice,
) -> CUresult;

pub const CU_DEVICE_GET_UUID_FN_NAME: &[u8] = b"cuDeviceGetUuid\0";
pub type CuDeviceGetUuidFn = unsafe extern "C" fn(uuid: *mut CUuuid, dev: CUdevice) -> CUresult;

pub const CU_CTX_CREATE_FN_NAME: &[u8] = b"cuCtxCreate_v2\0";
pub type CuCtxCreateFn = unsafe extern "C" fn(
    pctx: *mut CUcontext,
    flags: ::std::os::raw::c_uint,
    dev: CUdevice,
) -> CUresult;

pub const CU_CTX_DESTROY_FN_NAME: &[u8] = b"cuCtxDestroy_v2\0";
pub type CuCtxDestroyFn = unsafe extern "C" fn(ctx: CUcontext) -> CUresult;

pub const CU_CTX_PUSH_CURRENT_FN_NAME: &[u8] = b"cuCtxPushCurrent_v2\0";
pub type CuCtxPushCurrentFn = unsafe extern "C" fn(ctx: CUcontext) -> CUresult;

pub const CU_CTX_POP_CURRENT_FN_NAME: &[u8] = b"cuCtxPopCurrent_v2\0";
pub type CuCtxPopCurrentFn = unsafe extern "C" fn(pctx: *mut CUcontext) -> CUresult;

pub const CU_STREAM_CREATE_FN_NAME: &[u8] = b"cuStreamCreate\0";
pub type CuStreamCreateFn =
    unsafe extern "C" fn(phStream: *mut CUstream, flags: ::std::os::raw::c_uint) -> CUresult;

pub const CU_STREAM_DESTROY_FN_NAME: &[u8] = b"cuStreamDestroy_v2\0";
pub type CuStreamDestroyFn = unsafe extern "C" fn(hStream: CUstream) -> CUresult;

pub const CU_MEM_ALLOC_HOST_FN_NAME: &[u8] = b"cuMemAllocHost_v2\0";
pub type CuMemAllocHostFn =
    unsafe extern "C" fn(pp: *mut *mut ::std::os::raw::c_void, bytesize: usize) -> CUresult;

pub const CU_MEM_ALLOC_PITCH_FN_NAME: &[u8] = b"cuMemAllocPitch_v2\0";
pub type CuMemAllocPitchFn = unsafe extern "C" fn(
    dptr: *mut CUdeviceptr,
    pPitch: *mut usize,
    WidthInBytes: usize,
    Height: usize,
    ElementSizeBytes: ::std::os::raw::c_uint,
) -> CUresult;

pub const CU_MEM_FREE_FN_NAME: &[u8] = b"cuMemFree_v2\0";
pub type CuMemFreeFn = unsafe extern "C" fn(dptr: CUdeviceptr) -> CUresult;

pub const CU_MEM_FREE_HOST_FN_NAME: &[u8] = b"cuMemFreeHost\0";
pub type CuMemFreeHostFn = unsafe extern "C" fn(p: *mut ::std::os::raw::c_void) -> CUresult;

pub const CU_MEMCPY_2D_FN_NAME: &[u8] = b"cuMemcpy2D_v2\0";
pub type CuMemcpy2DFn = unsafe extern "C" fn(pCopy: *const CUDA_MEMCPY2D) -> CUresult;

pub const CU_MEMCPY_2D_UNALIGNED_FN_NAME: &[u8] = b"cuMemcpy2DUnaligned_v2\0";
pub type CuMemcpy2DUnalignedFn = unsafe extern "C" fn(pCopy: *const CUDA_MEMCPY2D) -> CUresult;

pub const CU_MEMCPY_2D_ASYNC_FN_NAME: &[u8] = b"cuMemcpy2DAsync_v2\0";
pub type CuMemcpy2DAsyncFn =
    unsafe extern "C" fn(pCopy: *const CUDA_MEMCPY2D, hStream: CUstream) -> CUresult;

pub const CU_MEMCPY_D_TO_H_FN_NAME: &[u8] = b"cuMemcpyDtoH_v2\0";
pub type CuMemcpyDtoHFn = unsafe extern "C" fn(
    dstHost: *mut ::std::os::raw::c_void,
    srcDevice: CUdeviceptr,
    ByteCount: usize,
) -> CUresult;

pub const CU_IMPORT_EXTERNAL_MEMORY_FN_NAME: &[u8] = b"cuImportExternalMemory\0";
pub type CuImportExternalMemoryFn = unsafe extern "C" fn(
    extMem_out: *mut CUexternalMemory,
    memHandleDesc: *const CUDA_EXTERNAL_MEMORY_HANDLE_DESC,
) -> CUresult;

pub const CU_IMPORT_EXTERNAL_SEMAPHORE_FN_NAME: &[u8] = b"cuImportExternalSemaphore\0";
pub type CuImportExternalSemaphoreFn = unsafe extern "C" fn(
    extSem_out: *mut CUexternalSemaphore,
    semHandleDesc: *const CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC,
) -> CUresult;

pub const CU_EXTERNAL_MEMORY_GET_MAPPED_BUFFER_FN_NAME: &[u8] =
    b"cuExternalMemoryGetMappedBuffer\0";
pub type CuExternalMemoryGetMappedBufferFn = unsafe extern "C" fn(
    evPtr: *mut CUdeviceptr,
    extMem: CUexternalMemory,
    bufferDesc: *const CUDA_EXTERNAL_MEMORY_BUFFER_DESC,
) -> CUresult;

pub const CU_EXTERNAL_MEMORY_GET_MAPPED_MIPMAPPED_ARRAY_FN_NAME: &[u8] =
    b"cuExternalMemoryGetMappedMipmappedArray\0";
pub type CuExternalMemoryGetMappedMipmappedArrayFn = unsafe extern "C" fn(
    mipmap: *mut CUmipmappedArray,
    extMem: CUexternalMemory,
    mipmapDesc: *const CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC,
) -> CUresult;

pub const CU_MIPMAPPED_ARRAY_GET_LEVEL_FN_NAME: &[u8] = b"cuMipmappedArrayGetLevel\0";
pub type CuMipmappedArrayGetLevelFn = unsafe extern "C" fn(
    pLevelArray: *mut CUarray,
    hMipmappedArray: CUmipmappedArray,
    level: ::std::os::raw::c_uint,
) -> CUresult;

pub const CU_MIPMAPPED_ARRAY_DESTROY_FN_NAME: &[u8] = b"cuMipmappedArrayDestroy\0";
pub type CuMipmappedArrayDestroyFn =
    unsafe extern "C" fn(hMipmappedArray: CUmipmappedArray) -> CUresult;

pub const CU_DESTROY_EXTERNAL_MEMORY_FN_NAME: &[u8] = b"cuDestroyExternalMemory\0";
pub type CuDestroyExternalMemoryFn = unsafe extern "C" fn(extMem: CUexternalMemory) -> CUresult;

pub const CU_DESTROY_EXTERNAL_SEMAPHORE_FN_NAME: &[u8] = b"cuDestroyExternalSemaphore\0";
pub type CuDestroyExternalSemaphoreFn =
    unsafe extern "C" fn(extSem: CUexternalSemaphore) -> CUresult;

pub const CU_WAIT_EXTERNAL_SEMAPHORES_ASYNC_FN_NAME: &[u8] = b"cuWaitExternalSemaphoresAsync\0";
pub type CuWaitExternalSemaphoresAsyncFn = unsafe extern "C" fn(
    extSemArray: *const CUexternalSemaphore,
    paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
    numExtSems: ::std::os::raw::c_uint,
    stream: CUstream,
) -> CUresult;

pub const CU_SIGNAL_EXTERNAL_SEMAPHORES_ASYNC_FN_NAME: &[u8] = b"cuSignalExternalSemaphoresAsync\0";
pub type CuSignalExternalSemaphoresAsyncFn = unsafe extern "C" fn(
    extSemArray: *const CUexternalSemaphore,
    paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
    numExtSems: ::std::os::raw::c_uint,
    stream: CUstream,
) -> CUresult;

#[cfg(windows)]
pub const CUDA_DLL_NAME: &str = "nvcuda.dll";

#[cfg(not(windows))]
pub const CUDA_DLL_NAME: &str = "libcuda.so";

#[cfg(test)]
mod tests {
    use crate::cuda::{
        CUresult, CuDeviceGetCountFn, CuInitFn, CUDA_DLL_NAME, CU_DEVICE_GET_COUNT_FN_NAME,
        CU_INIT_FN_NAME,
    };
    #[test]
    #[ignore = "needs nvidia gpu to test"]
    fn test_cuda() {
        let cuda_dll = unsafe { libloading::Library::new(CUDA_DLL_NAME).unwrap() };
        let cu_init_fn = unsafe {
            cuda_dll
                .get::<CuInitFn>(CU_INIT_FN_NAME)
                .expect("failed to load cuda init function")
        };
        let status = unsafe { cu_init_fn(0) };
        assert_eq!(status, CUresult::CUDA_SUCCESS);
        let cu_device_count_fn = unsafe {
            cuda_dll
                .get::<CuDeviceGetCountFn>(CU_DEVICE_GET_COUNT_FN_NAME)
                .expect("failed to load cuda init function")
        };
        let mut count: ::std::os::raw::c_int = 0;
        let status = unsafe { cu_device_count_fn(&mut count) };
        assert_eq!(count, 1);
        assert_eq!(status, CUresult::CUDA_SUCCESS);
    }
}

// This is here for correctness testing of the declared functions
mod validation {
    use super::*;
    const _CU_GET_ERROR_STRING: CuGetErrorStringFn = cuGetErrorString;
    const _CU_GET_ERROR_NAME: CuGetErrorNameFn = cuGetErrorName;
    const _CU_INIT: CuInitFn = cuInit;
    const _CU_DEVICE_GET_COUNT: CuDeviceGetCountFn = cuDeviceGetCount;
    const _CU_DEVICE_GET: CuDeviceGetFn = cuDeviceGet;
    const _CU_DEVICE_GET_NAME: CuDeviceGetNameFn = cuDeviceGetName;
    const _CU_DEVICE_GET_UUID: CuDeviceGetUuidFn = cuDeviceGetUuid;
    const _CU_CTX_CREATE: CuCtxCreateFn = cuCtxCreate_v2;
    const _CU_CTX_DESTROY: CuCtxDestroyFn = cuCtxDestroy_v2;
    const _CU_CTX_PUSH_CURRENT: CuCtxPushCurrentFn = cuCtxPushCurrent_v2;
    const _CU_CTX_POP_CURRENT: CuCtxPopCurrentFn = cuCtxPopCurrent_v2;
    const _CU_STREAM_CREATE: CuStreamCreateFn = cuStreamCreate;
    const _CU_STREAM_DESTROY: CuStreamDestroyFn = cuStreamDestroy_v2;
    const _CU_MEM_ALLOC_HOST: CuMemAllocHostFn = cuMemAllocHost_v2;
    const _CU_MEM_ALLOC_PITCH: CuMemAllocPitchFn = cuMemAllocPitch_v2;
    const _CU_MEM_FREE: CuMemFreeFn = cuMemFree_v2;
    const _CU_MEM_FREE_HOST: CuMemFreeHostFn = cuMemFreeHost;
    const _CU_MEM_COPY_2D: CuMemcpy2DFn = cuMemcpy2D_v2;
    const _CU_MEMCPY_2D_UNALIGNED: CuMemcpy2DUnalignedFn = cuMemcpy2DUnaligned_v2;
    const _CU_MEMCPY_2D_ASYNC: CuMemcpy2DAsyncFn = cuMemcpy2DAsync_v2;
    const _CU_MEMCPY_D_TO_H: CuMemcpyDtoHFn = cuMemcpyDtoH_v2;
    const _CU_IMPORT_EXTERNAL_SEMAPHORE: CuImportExternalSemaphoreFn = cuImportExternalSemaphore;
    const _CU_EXTERNAL_MEMORY_GET_MAPPED_BUFFER: CuExternalMemoryGetMappedBufferFn =
        cuExternalMemoryGetMappedBuffer;
    const _CU_EXTERNAL_MEMORY_GET_MAPPED_MIPMAPPED_ARRAY:
        CuExternalMemoryGetMappedMipmappedArrayFn = cuExternalMemoryGetMappedMipmappedArray;
    const _CU_MIPMAPPED_ARRAY_GET_LEVEL: CuMipmappedArrayGetLevelFn = cuMipmappedArrayGetLevel;
    const _CU_MIPMAPPED_ARRAY_DESTROY: CuMipmappedArrayDestroyFn = cuMipmappedArrayDestroy;
    const _CU_DESTROY_EXTERNAL_MEMORY: CuDestroyExternalMemoryFn = cuDestroyExternalMemory;
    const _CU_DESTROY_EXTERNAL_SEMAPHORE: CuDestroyExternalSemaphoreFn = cuDestroyExternalSemaphore;
    const _CU_WAIT_EXTERNAL_SEMAPHORES_ASYNC: CuWaitExternalSemaphoresAsyncFn =
        cuWaitExternalSemaphoresAsync;
    const _CU_SIGNAL_EXTERNAL_SEMAPHORES_ASYNC: CuSignalExternalSemaphoresAsyncFn =
        cuSignalExternalSemaphoresAsync;
}
