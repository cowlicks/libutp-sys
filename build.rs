use std::path::PathBuf;
use std::env;

fn main() {
    if cfg!(windows) {
        println!("cargo:rerun-if-changed=utp_win.h");
    } else {
        println!("cargo:rerun-if-changed=utp_posix.h");
    }

    compile();

    println!("cargo:rustc-link-lib=static=libutp");
    println!("cargo:rustc-link-search=native=build");

    let bindings = bindgen::Builder::default()
        .header(if cfg!(windows) {
            "utp_win.h"
        } else {
            "utp_posix.h"
        })
        .blocklist_type("sockaddr.*")
        .blocklist_type(".*INTERFACE_INFO.*")
        .blocklist_type(".*W2KSP1.*")
        .blocklist_type("LPMONITORINFOEXA?W?")
        .blocklist_type("LPTOP_LEVEL_EXCEPTION_FILTER")
        .blocklist_type("MONITORINFOEXA?W?")
        .blocklist_type("PEXCEPTION_FILTER")
        .blocklist_type("PEXCEPTION_ROUTINE")
        .blocklist_type("PSLIST_HEADER")
        .blocklist_type("PTOP_LEVEL_EXCEPTION_FILTER")
        .blocklist_type("PVECTORED_EXCEPTION_HANDLER")
        .blocklist_type("_?L?P?EXCEPTION_POINTERS")
        .blocklist_type("_?P?DISPATCHER_CONTEXT")
        .blocklist_type("_?P?EXCEPTION_REGISTRATION_RECORD")
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .blocklist_type("_?P?NT_TIB")
        .blocklist_type("tagMONITORINFOEXA")
        .blocklist_type("tagMONITORINFOEXW")
        .blocklist_function("AddVectoredContinueHandler")
        .blocklist_function("AddVectoredExceptionHandler")
        .blocklist_function("CopyContext")
        .blocklist_function("GetThreadContext")
        .blocklist_function("GetXStateFeaturesMask")
        .blocklist_function("InitializeContext")
        .blocklist_function("InitializeContext2")
        .blocklist_function("InitializeSListHead")
        .blocklist_function("InterlockedFlushSList")
        .blocklist_function("InterlockedPopEntrySList")
        .blocklist_function("InterlockedPushEntrySList")
        .blocklist_function("InterlockedPushListSListEx")
        .blocklist_function("LocateXStateFeature")
        .blocklist_function("QueryDepthSList")
        .blocklist_function("RaiseFailFastException")
        .blocklist_function("RtlCaptureContext")
        .blocklist_function("RtlCaptureContext2")
        .blocklist_function("RtlFirstEntrySList")
        .blocklist_function("RtlInitializeSListHead")
        .blocklist_function("RtlInterlockedFlushSList")
        .blocklist_function("RtlInterlockedPopEntrySList")
        .blocklist_function("RtlInterlockedPushEntrySList")
        .blocklist_function("RtlInterlockedPushListSListEx")
        .blocklist_function("RtlQueryDepthSList")
        .blocklist_function("RtlRestoreContext")
        .blocklist_function("RtlUnwindEx")
        .blocklist_function("RtlVirtualUnwind")
        .blocklist_function("SetThreadContext")
        .blocklist_function("SetUnhandledExceptionFilter")
        .blocklist_function("SetXStateFeaturesMask")
        .blocklist_function("UnhandledExceptionFilter")
        .blocklist_function("__C_specific_handler")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn compile() {
    let src = [
        "libutp/utp_api.cpp",
        "libutp/utp_callbacks.cpp",
        "libutp/utp_hash.cpp",
        "libutp/utp_internal.cpp",
        "libutp/utp_packedsockaddr.cpp",
        "libutp/utp_utils.cpp",
    ];


    let mut builder = cc::Build::new();

    builder.cpp(true)
        .include("libutp")
        .files(src.iter());

    if cfg!(windows) {
        builder
            .file("libutp/libutp_inet_ntop.cpp")
            .define("WIN32", None)
    } else {
        builder.define("POSIX", None)
    }
    .compile("libutp");
}

