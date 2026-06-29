// CUDA 13 renamed several driver API entry points that take struct parameters:
//   cuCtxCreate_v2   → cuCtxCreate_v4   (added CUctxCreateParams*)
//   cuEventElapsedTime → cuEventElapsedTime_v2
//
// cuda-rs needs both code paths. The CUDA_VERSION env var (set by the justfile
// as `CUDA_VERSION=13`) controls which path is compiled. When it equals `13`
// the `cuda_13` cfg is set, and the new function names are used. Without it
// (or when CUDA_VERSION is anything else, e.g. "12.6"), the old names are used.
//
// This is a compile-time decision: the actual CUDA driver library is linked at
// runtime and exports the right symbols for whichever version is installed.
fn main() {
    println!("cargo:rerun-if-env-changed=CUDA_VERSION");
    let cuda_ver = std::env::var("CUDA_VERSION").unwrap_or_default();
    if cuda_ver == "13" {
        println!("cargo:rustc-cfg=cuda_13");
    }
}
