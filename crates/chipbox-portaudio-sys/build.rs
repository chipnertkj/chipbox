use std::path::{Path, PathBuf};

use chipbox_build::{
    build_script, lockfile,
    miette::{self, Context as _, IntoDiagnostic as _},
};

#[cfg(all(windows, feature = "asio"))]
mod asio;

fn main() -> miette::Result<()> {
    build_init().wrap_err("init")?;
    #[cfg(all(windows, feature = "asio"))]
    asio::setup_sdk().wrap_err("setup asio sdk")?;
    let portaudio_path = chipbox_build::fs::portaudio()
        .into_diagnostic()
        .wrap_err("portaudio path")?;
    #[cfg(all(windows, feature = "asio"))]
    let asio_path = Some(
        asio::target_subdir()
            .wrap_err("asio target subdir")?
            .join(asio::sdk_extracted_dir_name()),
    );
    #[cfg(not(all(windows, feature = "asio")))]
    let asio_path = Option::<&'static Path>::None;

    let build = std::thread::spawn({
        let portaudio_path = portaudio_path.clone();
        move || build_portaudio(portaudio_path, asio_path.as_ref())
    });
    let bindings = std::thread::spawn(move || generate_bindings(portaudio_path));

    build
        .join()
        .expect("build portaudio panic")
        .wrap_err("build portaudio")?;
    bindings
        .join()
        .expect("generate bindings panic")
        .wrap_err("generate bindings")?;

    Ok(())
}

fn build_init() -> miette::Result<()> {
    build_script::rerun_on_script_change();
    lockfile::assert_deps_consistency(&lockfile::load_workspace()?)?;
    Ok(())
}

fn generate_bindings(portaudio_path: impl AsRef<Path>) -> miette::Result<()> {
    let portaudio_path = portaudio_path.as_ref();
    let bindings = bindgen::Builder::default()
        .header(portaudio_path.join("include/portaudio.h").to_string_lossy())
        .generate()
        .into_diagnostic()
        .wrap_err("generate bindings")?;
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .into_diagnostic()
        .wrap_err("write bindings")?;
    Ok(())
}

fn build_portaudio(
    portaudio_path: impl AsRef<Path>,
    asio_path: Option<impl AsRef<Path>>,
) -> miette::Result<()> {
    let portaudio_path = portaudio_path.as_ref();
    let mut build = cc::Build::new();

    // Common source files.
    let common_sources = [
        portaudio_path.join("src/common/pa_allocation.c"),
        portaudio_path.join("src/common/pa_converters.c"),
        portaudio_path.join("src/common/pa_cpuload.c"),
        portaudio_path.join("src/common/pa_debugprint.c"),
        portaudio_path.join("src/common/pa_dither.c"),
        portaudio_path.join("src/common/pa_front.c"),
        portaudio_path.join("src/common/pa_process.c"),
        portaudio_path.join("src/common/pa_ringbuffer.c"),
        portaudio_path.join("src/common/pa_stream.c"),
        portaudio_path.join("src/common/pa_trace.c"),
    ];

    // Common includes.
    let common_includes = [
        portaudio_path.join("src/common"),
        portaudio_path.join("include"),
        portaudio_path.join("src/os/win"),
    ];

    // Windows-specific sources.
    #[cfg(windows)]
    let platform_sources = [
        portaudio_path.join("src/os/win/pa_win_coinitialize.c"),
        portaudio_path.join("src/os/win/pa_win_hostapis.c"),
        portaudio_path.join("src/os/win/pa_win_util.c"),
        portaudio_path.join("src/os/win/pa_win_waveformat.c"),
        portaudio_path.join("src/os/win/pa_win_wdmks_utils.c"),
        portaudio_path.join("src/os/win/pa_x86_plain_converters.c"),
        // Host APIs.
        portaudio_path.join("src/hostapi/wmme/pa_win_wmme.c"),
        portaudio_path.join("src/hostapi/wasapi/pa_win_wasapi.c"),
        portaudio_path.join("src/hostapi/wdmks/pa_win_wdmks.c"),
    ];
    #[cfg(not(windows))]
    let platform_sources = [];

    // Windows-specific includes.
    #[cfg(windows)]
    let platform_includes = [
        // TODO: gnu only
        //portaudio_path.join("src/hostapi/wasapi/mingw-include"),
    ];
    #[cfg(not(windows))]
    let platform_includes = [];

    // Add all sources.
    common_sources
        .into_iter()
        .chain(platform_sources)
        .for_each(|src| {
            build.file(src);
        });

    // Add all includes.
    common_includes
        .into_iter()
        .chain(platform_includes)
        .for_each(|inc| {
            build.include(inc);
        });

    // Windows-specific configuration.
    #[cfg(windows)]
    {
        build
            .define("WIN32", None)
            .define("_USRDLL", None)
            .define("_CRT_SECURE_NO_DEPRECATE", None)
            .define("PAWIN_USE_WDMKS_DEVICE_INFO", None)
            .define("PA_USE_DS", "0")
            .define("PA_USE_WMME", "1")
            .define("PA_USE_WASAPI", "1")
            .define("PA_USE_WDMKS", "1") // dx???
            .static_crt(true);

        #[cfg(target_pointer_width = "64")]
        build.define("_WIN64", None);
    }

    #[cfg(all(windows, feature = "asio"))]
    {
        let toolchain = build_script::toolchain();
        println!("cargo:warning={toolchain}");
        if !toolchain.is_msvc() {
            // panic!("ASIO is only supported on MSVC");
        }

        let asio_path = asio_path.expect("must provide asio path");
        let asio_path = asio_path.as_ref();

        // ASIO C++ sources
        let asio_sources = [
            portaudio_path.join("src/hostapi/asio/pa_asio.cpp"),
            portaudio_path.join("src/hostapi/asio/iasiothiscallresolver.cpp"),
            asio_path.join("common/asio.cpp"),
            asio_path.join("host/asiodrivers.cpp"),
            asio_path.join("host/pc/asiolist.cpp"),
        ];

        let asio_includes = [
            asio_path.to_owned(),
            asio_path.join("common"),
            asio_path.join("host"),
            asio_path.join("host/pc"),
        ];

        // Add ASIO sources.
        asio_sources.into_iter().for_each(|src| {
            eprintln!("{src:?}");
            build.file(src);
        });
        // Add ASIO include directories.
        asio_includes.into_iter().for_each(|inc| {
            eprintln!("{inc:?}");
            build.include(inc);
        });

        build
            .define("PA_USE_ASIO", "1")
            .flag_if_supported("-fpermissive")
            .cpp(true); // Enable C++ compilation.
    }

    // Optimization settings for release.
    {
        build
            .opt_level(2)
            .flag_if_supported("/Oi") // Enable intrinsic functions (MSVC).
            .flag_if_supported("/Ot"); // Favor speed (MSVC).
    }

    build.warnings(false); // Suppress warnings from third-party code.

    build.compile("portaudio");

    println!("cargo:warning=finished build");
    println!("cargo:rustc-link-lib=portaudio");

    // Link required Windows libraries.
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-lib=ksuser");
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=advapi32");
    }

    Ok(())
}
