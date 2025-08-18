use std::env;
use std::ffi::{OsStr, OsString};
use std::path::Path;

extern crate bindgen;

fn configure_diligent_libs(diligent_install_dir: &Path) {
    #[cfg(debug_assertions)]
    println!(
        "cargo::rustc-link-search={}/lib/DiligentCore/Debug",
        diligent_install_dir.display(),
    );

    #[cfg(not(debug_assertions))]
    println!(
        "cargo::rustc-link-search={}/lib/DiligentCore/Release",
        diligent_install_dir.display()
    );

    println!("cargo::rustc-link-lib=static=DiligentCore");

    #[cfg(all(debug_assertions, target_os = "windows"))]
    let library_suffix = "d";
    #[cfg(any(not(debug_assertions), not(target_os = "windows")))]
    let library_suffix = "";

    #[cfg(feature = "vulkan")]
    {
        println!("cargo::rustc-link-lib=static=glslang{library_suffix}");
        println!("cargo::rustc-link-lib=static=SPIRV{library_suffix}");
        println!("cargo::rustc-link-lib=static=SPIRV-Tools");
        println!("cargo::rustc-link-lib=static=SPIRV-Tools-opt");
        println!("cargo::rustc-link-lib=static=spirv-cross-core{library_suffix}");

        println!("cargo::rustc-link-lib=static=volk");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo::rustc-link-lib=ucrt{library_suffix}");
    }

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
}

fn generate_diligent_c_bindings(
    #[allow(unused_variables)] diligent_source_dir: &Path,
    diligent_install_dir: &Path,
    out_dir: &Path,
) {
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

    #[cfg(feature = "vulkan")]
    let configure_vulkan = |builder: bindgen::Builder| -> bindgen::Builder {
        let builder = builder.clang_arg("-DVULKAN_SUPPORTED=1");

        #[cfg(feature = "vulkan_interop")]
        let builder = builder.clang_arg("-DVULKAN_INTEROP=1").clang_arg(format!(
            "-I{}/DiligentCore/ThirdParty/Vulkan-Headers/include",
            diligent_source_dir.display()
        ));

        builder
    };

    #[cfg(feature = "opengl")]
    fn configure_opengl(builder: bindgen::Builder) -> bindgen::Builder {
        let builder = builder.clang_arg("-DOPENGL_SUPPORTED=1");

        #[cfg(feature = "opengl_interop")]
        let builder = builder.clang_arg("-DOPENGL_INTEROP=1");

        #[cfg(target_os = "linux")]
        {
            println!("cargo::rustc-link-lib=GL");
            println!("cargo::rustc-link-lib=X11");
            println!("cargo::rustc-link-lib=GLEW");
        }

        #[cfg(target_os = "windows")]
        {
            println!("cargo::rustc-link-lib=opengl32");
            println!("cargo::rustc-link-lib=glew-static");
        }

        builder
    }

    #[cfg(feature = "d3d11")]
    fn configure_d3d11(builder: bindgen::Builder) -> bindgen::Builder {
        let builder = builder.clang_arg("-DD3D11_SUPPORTED=1");

        #[cfg(feature = "d3d11_interop")]
        let builder = builder.clang_arg("-DD3D11_INTEROP=1");

        builder
    }

    #[cfg(feature = "d3d12")]
    fn configure_d3d12(builder: bindgen::Builder) -> bindgen::Builder {
        let builder = builder.clang_arg("-DD3D12_SUPPORTED=1");

        #[cfg(feature = "d3d12_interop")]
        let builder = builder.clang_arg("-DD3D12_INTEROP=1");

        builder
    }

    #[cfg(feature = "vulkan")]
    let builder = configure_vulkan(builder);

    #[cfg(feature = "opengl")]
    let builder = configure_opengl(builder);

    #[cfg(feature = "d3d11")]
    let builder = configure_d3d11(builder);

    #[cfg(feature = "d3d12")]
    let builder = configure_d3d12(builder);

    let bindings = builder.generate().expect("Unable to generate bindings");

    let diligent_bindings_filename = "diligent_bindings.rs";

    bindings
        .write_to_file(out_dir.join(diligent_bindings_filename))
        .unwrap_or_else(|_| {
            panic!("Unable to write bindings to file {diligent_bindings_filename}")
        });
}

fn get_env_os(env_name: impl AsRef<OsStr>) -> OsString {
    env::var_os(&env_name).unwrap_or_else(|| {
        panic!(
            "Could not find \"{}\" environment variable. Please add it your cargo config",
            env_name.as_ref().to_str().unwrap()
        )
    })
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);

    let diligent_source_dir = get_env_os("DILIGENT_SOURCE_DIR");
    let diligent_install_dir = get_env_os("DILIGENT_INSTALL_DIR");

    let diligent_source_dir = Path::new(&diligent_source_dir);
    let diligent_install_dir = Path::new(&diligent_install_dir);

    configure_diligent_libs(Path::new(&diligent_install_dir));

    generate_diligent_c_bindings(diligent_source_dir, diligent_install_dir, out_dir_path);
}
