use std::path::Path;
use std::path::PathBuf;
use std::env;

extern crate cmake;
extern crate bindgen;

fn build_diligent_engine(build_path: &PathBuf, install_prefix : &str) -> PathBuf
{
	let dst = cmake::Config::new("DiligentEngine")
	.out_dir(build_path)
	.define("CMAKE_INSTALL_PREFIX", install_prefix)
	.define("OpenGL_GL_PREFERENCE", "GLVND")
	.define("DILIGENT_BUILD_SAMPLES", "OFF")
	.define("DILIGENT_BUILD_FX", "OFF")
	.build().join("build/install");

	println!("cargo::rustc-link-search=native={}/lib/DiligentCore/Debug", dst.display());
	println!("cargo::rustc-link-lib=static=DiligentCore");

	dst
}

fn generate_diligent_c_bindings(diligent_install_dir : &PathBuf, out_dir : &PathBuf)
{
	let diligent_include = ["-I", &diligent_install_dir.join("include").display().to_string()].concat();

	let builder =
	{
		let builder = bindgen::Builder::default()
		.clang_arg(diligent_include)
		.header("wrapper.h").prepend_enum_name(false);

		match std::env::consts::OS
		{
			"windows" => builder.clang_arg("-DPLATFORM_WIN32=1"),
			"linux"   => builder.clang_arg("-DPLATFORM_LINUX=1"),
			"macos"   => builder.clang_arg("-DPLATFORM_MACOS=1"),
			_         => panic!("Unknown platform")
		}
	};

	let bindings = builder.generate().expect("Unable to generate bindings");

	let diligent_bindings_filename = "diligent_bindings.rs";

	bindings.write_to_file(out_dir.join(diligent_bindings_filename)).expect(format!("Unable to write bindings to file {}", diligent_bindings_filename).as_str());
}

fn main()
{
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let out_dir_path = Path::new(&out_dir);

	let diligent_build_dir = out_dir_path.join("DiligentEngine");
	let diligent_install_prefix = "install";

	let diligent_install_path = build_diligent_engine(&diligent_build_dir, diligent_install_prefix);

	generate_diligent_c_bindings(&diligent_install_path, &out_dir_path.to_path_buf());
}