#[cfg(feature = "use-bindgen")]
fn main() -> std::io::Result<()> {
    let out_dir = if std::env::var("LOCAL_COPY").is_ok() {
        std::path::PathBuf::from("./src/bindgen")
    } else {
        std::path::PathBuf::from(
            std::env::var("OUT_DIR").expect("Cargo build scripts always have OUT_DIR defined"),
        )
    };
    let nvenc_header_path = "nvenc/nvEncodeAPI.h";
    let nvenc_output_path = out_dir.join("nvenc.rs");
    bindgen::builder()
        .header(nvenc_header_path)
        .raw_line("// Generated through nvenc-sys build script")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("NV.*")
        .allowlist_function("Nv.*")
        .allowlist_var("NVENC.*")
        .allowlist_var("NV_MAX.*")
        .size_t_is_usize(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&nvenc_output_path)
        .expect("Unable to write bindings");

    // Generate additional defines for some macro definitions defining constants
    // #define NV_ENC_RECONFIGURE_PARAMS_VER (NVENCAPI_STRUCT_VERSION(1) | ( 1<<31 ))
    // #define NV_ENC_PIC_PARAMS_MVC_VER NVENCAPI_STRUCT_VERSION(1)
    let nvenc_header = std::fs::read_to_string(nvenc_header_path)?;
    let struct_ver_re = regex::Regex::new(
        r"#define\s+([A-Z_]+)\s+\(?NVENCAPI_STRUCT_VERSION\((\d)\)(?:\s*\|\s*\(([\s\d<]+)\))?\)?",
    )
    .unwrap();
    let mut extra_definitions = String::from("\n");
    extra_definitions.push_str("const fn nv_struct_version(ver: u32) -> u32 {\n");
    extra_definitions.push_str("    NVENCAPI_VERSION | ((ver) << 16) | (0x7 << 28)\n");
    extra_definitions.push_str("}\n");
    for caps in struct_ver_re.captures_iter(&nvenc_header) {
        let name = caps.get(1).unwrap().as_str();
        let value = caps.get(2).unwrap().as_str();
        if let Some(extra_bits) = caps.get(3) {
            extra_definitions.push_str(
                format!(
                    "pub const {}: u32 = nv_struct_version({}) | ({});\n",
                    name,
                    value,
                    extra_bits.as_str()
                )
                .as_str(),
            );
        } else {
            extra_definitions.push_str(
                format!("pub const {}: u32 = nv_struct_version({});\n", name, value).as_str(),
            );
        }
    }
    // Generate all guid definitions that are defined as static const
    // static const GUID NV_ENC_CODEC_H264_GUID =
    // { 0x6bc82762, 0x4e63, 0x4ca4, { 0xaa, 0x85, 0x1e, 0x50, 0xf3, 0x21, 0xf6, 0xbf } };
    let guid_re = regex::Regex::new(
        r"static\s+const\s+GUID\s+([A-Z_\d]+)\s*=\s*\r?\n\{\s*(0[xX][0-9a-fA-F]+)\s*,\s*(0[xX][0-9a-fA-F]+)\s*,\s*(0[xX][0-9a-fA-F]+)\s*,\s*\{\s*(0[xX][0-9a-fA-F]+)\s*,\s*(0[xX][0-9a-fA-F]+)\s*,\s*(0[xX][0-9a-fA-F]+)\s*,\s*(0[xX][0-9a-fA-F]+)\s*,\s*(0[xX][0-9a-fA-F]+)\s*,\s*(0[xX][0-9a-fA-F]+)\s*,\s*(0[xX][0-9a-fA-F]+)\s*,\s*(0[xX][0-9a-fA-F]+)\s*\}\s*\}\s*;"
    ).unwrap();
    for caps in guid_re.captures_iter(&nvenc_header) {
        let name = caps.get(1).unwrap().as_str();
        let data1 = caps.get(2).unwrap().as_str();
        let data2 = caps.get(3).unwrap().as_str();
        let data3 = caps.get(4).unwrap().as_str();
        let data4_1 = caps.get(5).unwrap().as_str();
        let data4_2 = caps.get(6).unwrap().as_str();
        let data4_3 = caps.get(7).unwrap().as_str();
        let data4_4 = caps.get(8).unwrap().as_str();
        let data4_5 = caps.get(9).unwrap().as_str();
        let data4_6 = caps.get(10).unwrap().as_str();
        let data4_7 = caps.get(11).unwrap().as_str();
        let data4_8 = caps.get(12).unwrap().as_str();
        extra_definitions.push_str(format!("pub const {}: GUID = GUID {{\n", name).as_str());
        extra_definitions.push_str(format!("    Data1: {},\n", data1).as_str());
        extra_definitions.push_str(format!("    Data2: {},\n", data2).as_str());
        extra_definitions.push_str(format!("    Data3: {},\n", data3).as_str());
        extra_definitions.push_str(
            format!(
                "    Data4: [{}, {}, {}, {}, {}, {}, {}, {}],\n}};\n",
                data4_1, data4_2, data4_3, data4_4, data4_5, data4_6, data4_7, data4_8
            )
            .as_str(),
        );
    }
    use std::io::Write;
    std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(nvenc_output_path)
        .unwrap()
        .write_all(extra_definitions.as_bytes())?;

    if cfg!(feature = "cuda") {
        let cuda_header_path = format!(
            "{}/include/cuda.h",
            std::env::var("CUDA_PATH").expect("CUDA_PATH must be set")
        );
        bindgen::builder()
            .header(cuda_header_path)
            .raw_line("// Generated through nvenc-sys build script")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .allowlist_function("cuGetErrorString")
            .allowlist_function("cuGetErrorName")
            .allowlist_function("cuInit")
            .allowlist_function("cuDeviceGetCount")
            .allowlist_function("cuDeviceGet")
            .allowlist_function("cuDeviceGetName")
            .allowlist_function("cuDeviceGetUuid")
            .allowlist_function("cuCtxCreate_v2")
            .allowlist_function("cuCtxDestroy_v2")
            .allowlist_function("cuCtxPushCurrent_v2")
            .allowlist_function("cuCtxPopCurrent_v2")
            .allowlist_function("cuStreamCreate")
            .allowlist_function("cuStreamDestroy_v2")
            .allowlist_function("cuMemAllocHost_v2")
            .allowlist_function("cuMemAllocPitch_v2")
            .allowlist_function("cuMemFree_v2")
            .allowlist_function("cuMemFreeHost")
            .allowlist_function("cuMemcpy2D_v2")
            .allowlist_function("cuMemcpy2DUnaligned_v2")
            .allowlist_function("cuMemcpy2DAsync_v2")
            .allowlist_function("cuMemcpyDtoH_v2")
            .allowlist_function("cuImportExternalMemory")
            .allowlist_function("cuImportExternalSemaphore")
            .allowlist_function("cuExternalMemoryGetMappedBuffer")
            .allowlist_function("cuExternalMemoryGetMappedMipmappedArray")
            .allowlist_function("cuMipmappedArrayGetLevel")
            .allowlist_function("cuMipmappedArrayDestroy")
            .allowlist_function("cuDestroyExternalMemory")
            .allowlist_function("cuDestroyExternalSemaphore")
            .allowlist_function("cuWaitExternalSemaphoresAsync")
            .allowlist_function("cuSignalExternalSemaphoresAsync")
            .size_t_is_usize(true)
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: false,
            })
            .generate_comments(false)
            .derive_default(true)
            .derive_eq(true)
            .derive_hash(true)
            .derive_ord(true)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(&out_dir.join("cuda.rs"))
            .expect("Unable to write bindings");

        println!("cargo:rerun-if-env-changed=CUDA_PATH");
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LOCAL_COPY");

    Ok(())
}

#[cfg(not(feature = "use-bindgen"))]
fn main() -> std::io::Result<()> {
    Ok(())
}
