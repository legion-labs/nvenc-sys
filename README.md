[![CI](https://github.com/legion-labs/nvenc-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/legion-labs/amf-rs/actions/workflows/ci.yml)

# NVENC Rust FFI bindings

This repo contains bindings for the [NVENC library](https://developer.nvidia.com/nvidia-video-codec-sdk). The current bindings target mainly encoding workflows, decoding might be added later (PR welcomed).

The bindings are [pre generated](https://legionlabs.com/blog/rust_ffi_finding/) using bindgen with some extra regexes for some macros and static declarations.

This crate also exposes bindings for a very small subset of Nvidia's CUDA driver library, only the necessary functions/structures/enum to interact with the encoder as a straightforward way to load a stream of surfaces and encode them and to interop with Vulkan is covered.

## Regenerating bindings

The bindings are generated using [bindgen](https://crates.io/crates/bindgen) which depends on a libclang. Bindgen by default looks for it in these folders on windows:

- C:\\LLVM
- C:\\Program Files\*\\LLVM
- C:\\MSYS*\\MinGW*
- C:\\Program Files*\\Microsoft Visual Studio\\*\\BuildTools\\VC\\Tools\\Llvm

Otherwise you can manually set LIBCLANG_PATH environment variable to point to libclang.dll, or use the legion-labs scoop bucket to install llvm:

```powershell
scoop install llvm
```

If the cuda feature is enabled (allows decoding), you need to install the [CUDA Toolkit](https://developer.nvidia.com/cuda-downloads).
Or on windows

```
scoop install cuda
```

On linux you can follow the installation guide depending on your distribution for llvm and cuda.

Use the `use-bindgen` feature to use dynamically generated bindings. To regenerate the bundled bindings set the `LOCAL_COPY` environment variable along with the `use-bindgen` feature.

To run some basic loading tests `cargo test [--features=cuda] -- --ignored`, you need to have an Nvidia graphic card with hardware encoding capabilities and proper drivers to pass the tests [(refer to Nvidia's documentation for support matrix)](https://developer.nvidia.com/video-encode-and-decode-gpu-support-matrix-new).

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Refer to [License agreement](https://docs.nvidia.com/video-technologies/video-codec-sdk/license/index.html) for the use of the NVENC library. And you should check the [End User License Agreement](https://docs.nvidia.com/cuda/eula/index.html), which describes NVIDIA Software License Agreement and CUDA Supplement to Software License Agreement.

The library bundles the nvEncodeAPI.h header file from the NVENC SDK, please refer to the copyright notice at the top of the file for further information about the header distribution.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
