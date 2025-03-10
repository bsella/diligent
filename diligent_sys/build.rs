use std::env;
use std::path::Path;
use std::path::PathBuf;

extern crate bindgen;
extern crate cmake;

fn configure_cmake_windows_debug(cmake_config: &mut cmake::Config) {
    //cmake_config.profile("Debug");
    //cmake_config.static_crt(true);
    cmake_config.no_default_flags(true);
    cmake_config.cflag("/MDd /Zi /Ob0 /Od /RTC1");
    cmake_config.cxxflag("/MDd /Zi /Ob0 /Od /RTC1");

    println!("cargo::rustc-link-lib=ucrtd");
}

fn build_diligent_engine(build_path: &PathBuf, install_prefix: &str) -> PathBuf {
    let mut cmake_config = cmake::Config::new("DiligentEngine");

    #[cfg(all(debug_assertions, target_os = "windows"))]
    configure_cmake_windows_debug(&mut cmake_config);

    //#[cfg(not(debug_assertions))]
    //cmake_config.profile("Release");

    cmake_config
        .out_dir(build_path)
        .define("CMAKE_INSTALL_PREFIX", install_prefix)
        .define("OpenGL_GL_PREFERENCE", "GLVND")
        .define("DILIGENT_BUILD_SAMPLES", "OFF")
        .define("DILIGENT_BUILD_FX", "OFF")
        .define("DILIGENT_BUILD_TOOLS", "OFF")
        .define("DILIGENT_NO_ARCHIVER", "ON");

    let dst = cmake_config.build().join("build/install");

    #[cfg(debug_assertions)]
    println!(
        "cargo::rustc-link-search=native={}/lib/DiligentCore/Debug",
        dst.display()
    );

    #[cfg(not(debug_assertions))]
    println!(
        "cargo::rustc-link-search=native={}/lib/DiligentCore/Release",
        dst.display()
    );

    #[cfg(debug_assertions)]
    let library_suffix = "d";
    #[cfg(not(debug_assertions))]
    let library_suffix = "";

    println!("cargo::rustc-link-lib=static=DiligentCore");
    println!("cargo::rustc-link-lib=static=glslang{library_suffix}");
    println!("cargo::rustc-link-lib=static=SPIRV{library_suffix}");
    println!("cargo::rustc-link-lib=static=SPIRV-Tools");
    println!("cargo::rustc-link-lib=static=SPIRV-Tools-opt");
    println!("cargo::rustc-link-lib=static=spirv-cross-core{library_suffix}");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    dst
}

fn generate_diligent_c_bindings(diligent_install_dir: &PathBuf, out_dir: &PathBuf) {
    let diligent_include = [
        "-I",
        &diligent_install_dir.join("include").display().to_string(),
    ]
    .concat();

    let builder = {
        let builder = bindgen::Builder::default()
            .clang_arg(diligent_include)
            .header("wrapper.h")
            .prepend_enum_name(false);

        match std::env::consts::OS {
            "windows" => builder.clang_arg("-DPLATFORM_WIN32=1"),
            "linux" => builder.clang_arg("-DPLATFORM_LINUX=1"),
            "macos" => builder.clang_arg("-DPLATFORM_MACOS=1"),
            _ => panic!("Unknown platform"),
        }
    };

    fn configure_vulkan(builder: bindgen::Builder) -> bindgen::Builder {
        let builder = builder.clang_arg("-DVULKAN_SUPPORTED=1");

        #[cfg(feature = "vulkan_interop")]
        let builder = builder
            .clang_arg("-DVULKAN_INTEROP=1")
            .clang_arg("-IDiligentEngine/DiligentCore/ThirdParty/Vulkan-Headers/include");

        builder
    }

    #[cfg(feature = "vulkan")]
    let builder = configure_vulkan(builder);

    let bindings = builder.generate().expect("Unable to generate bindings");

    let diligent_bindings_filename = "diligent_bindings.rs";

    bindings
        .write_to_file(out_dir.join(diligent_bindings_filename))
        .expect(
            format!(
                "Unable to write bindings to file {}",
                diligent_bindings_filename
            )
            .as_str(),
        );
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);

    let diligent_build_dir = out_dir_path.join("DiligentEngine");
    let diligent_install_prefix = "install";

    let diligent_install_path = build_diligent_engine(&diligent_build_dir, diligent_install_prefix);

    generate_diligent_c_bindings(&diligent_install_path, &out_dir_path.to_path_buf());
}
