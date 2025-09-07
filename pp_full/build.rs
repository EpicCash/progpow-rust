extern crate bindgen;
extern crate cmake;
extern crate filetime;

use filetime::FileTime;

use std::env;
use std::fs;

pub fn fail_on_empty_directory(name: &str) {
	if fs::read_dir(name).unwrap().count() == 0 {
		println!(
			"The `{}` directory is empty. Did you forget to pull the submodules?",
			name
		);
		println!("Try `git submodule update --init --recursive`");
		panic!();
	}
}

fn generate_bindings(out_dir: &str) {
	let bindings = bindgen::Builder::default()
		.header("lib/libexternal/progpow.h")
		.blacklist_type("max_align_t")
		.blacklist_type("_bindgen_ty_1")
		.generate()
		.expect("Unable to generate bindings");

	//let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(format!("{}/ffi.rs", out_dir))
		.expect("Couldn't write bindings!");
}

fn compile_cmake() {
	let mut make = cmake::Config::new("lib");

	if cfg!(feature = "cuda") {
		make.define("ETHASHCUDA", "ON");
	} else {
		make.define("ETHASHCUDA", "OFF");
	}

	if cfg!(feature = "opencl") {
		make.define("ETHASHCL", "ON");
	} else {
		make.define("ETHASHCL", "OFF");
	}

	make.no_build_target(true).build();
}

fn exec_if_newer<F: Fn()>(inpath: &str, outpath: &str, build: F) {
	if let Ok(metadata) = fs::metadata(outpath) {
		let outtime = FileTime::from_last_modification_time(&metadata);
		let intime = FileTime::from_last_modification_time(
			&fs::metadata(inpath).expect(&format!("Path {} not found", inpath)),
		);
		let buildfiletime =
			FileTime::from_last_modification_time(&fs::metadata("build.rs").unwrap());
		if outtime > intime && outtime > buildfiletime {
			return;
		}
	}
	build();
}

fn main() {
	println!("Starting progpow build");

	let out_dir = env::var("OUT_DIR").unwrap();

	fail_on_empty_directory("lib");

	compile_cmake();

	if cfg!(target_env = "msvc") {
		let target = if cfg!(debug_assertions) {
			"Debug"
		} else {
			"Release"
		};

		if cfg!(feature = "opencl") {
			println!(
				"cargo:rustc-link-search={}/build/libethash-cl/{}",
				out_dir, target
			);
			println!("cargo:rustc-link-lib=ethash-cl");
		}

		if cfg!(feature = "cuda") {
			println!(
				"cargo:rustc-link-search={}/build/libethash-cl/{}",
				out_dir, target
			);
			println!("cargo:rustc-link-lib=ethash-cuda");
		}

		println!(
			"cargo:rustc-link-search={}/build/libethash/{}",
			out_dir, target
		);
		println!("cargo:rustc-link-lib=ethash");
		println!(
			"cargo:rustc-link-search={}/build/libprogpow/{}",
			out_dir, target
		);
		println!("cargo:rustc-link-lib=progpow");
		println!(
			"cargo:rustc-link-search={}/build/libethcore/{}",
			out_dir, target
		);
		println!("cargo:rustc-link-lib=ethcore");
		println!(
			"cargo:rustc-link-search={}/build/libdevcore/{}",
			out_dir, target
		);
		println!("cargo:rustc-link-lib=devcore");
		println!(
			"cargo:rustc-link-search={}/build/libexternal/{}",
			out_dir, target
		);
		println!("cargo:rustc-link-lib=ppow");
		println!("cargo:rustc-link-lib=OpenCL");
	} else {
		println!("cargo:rustc-link-search={}/build/libexternal", out_dir);
		println!("cargo:rustc-link-lib=ppow");
	}
}
