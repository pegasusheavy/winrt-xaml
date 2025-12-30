use std::fs;
use std::path::Path;

fn main() {
    // Embed Windows manifest for XAML Islands support using winres
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("winrt-xaml.manifest");
        res.compile().unwrap();
    }

    // Tell Cargo where to find the xaml_islands_helper DLL
    println!("cargo:rustc-link-search=native=target/debug");
    println!("cargo:rustc-link-search=native=xaml_islands_helper/build/bin/Debug");
    println!("cargo:rustc-link-search=native=xaml_islands_helper/build/Debug");

    // Copy DLL to runtime locations
    let dll_source = "target/debug/xaml_islands_helper.dll";
    if Path::new(dll_source).exists() {
        // Copy to x86_64-pc-windows-msvc target directory
        let _ = fs::create_dir_all("target/x86_64-pc-windows-msvc/debug");
        let _ = fs::copy(dll_source, "target/x86_64-pc-windows-msvc/debug/xaml_islands_helper.dll");

        // Copy to examples directory
        let _ = fs::create_dir_all("target/x86_64-pc-windows-msvc/debug/examples");
        let _ = fs::copy(dll_source, "target/x86_64-pc-windows-msvc/debug/examples/xaml_islands_helper.dll");
    }

    // Rerun if DLL changes
    println!("cargo:rerun-if-changed=xaml_islands_helper/build/bin/Debug/xaml_islands_helper.dll");
    println!("cargo:rerun-if-changed=target/debug/xaml_islands_helper.dll");
    println!("cargo:rerun-if-changed=winrt-xaml.manifest");
}
