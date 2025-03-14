use heck::ToSnakeCase;
use r2r_common::RosMsg;

use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::{env, fs};

const MSG_INCLUDES_FILENAME: &str = "msg_includes.h";
const INTROSPECTION_FILENAME: &str = "introspection_functions.rs";
const BINDINGS_FILENAME: &str = "msg_bindings.rs";
const GENERATED_FILES: &[&str] = &[
    MSG_INCLUDES_FILENAME,
    INTROSPECTION_FILENAME,
    BINDINGS_FILENAME,
];

fn main() {
    r2r_common::print_cargo_watches();

    let msg_list = r2r_common::get_wanted_messages();
    run_bindgen(&msg_list);
    run_dynlink(&msg_list);
}

fn run_bindgen(msg_list: &[RosMsg]) {
    let env_hash = r2r_common::get_env_hash();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let bindgen_dir = out_dir.join(env_hash);
    let mark_file = bindgen_dir.join("done");
    let save_dir = manifest_dir.join("bindings");

    if cfg!(feature = "doc-only") {
        // If "doc-only" feature is present, copy from $crate/bindings/* to OUT_DIR
        eprintln!(
            "Copy files from '{}' to '{}'",
            save_dir.display(),
            out_dir.display()
        );

        for filename in GENERATED_FILES {
            let src = save_dir.join(filename);
            let tgt = out_dir.join(filename);
            fs::copy(&src, &tgt).unwrap();
        }
    } else {
        // If bindgen was done before, use cached files.
        if !mark_file.exists() {
            eprintln!("Generate bindings in '{}'", bindgen_dir.display());
            fs::create_dir_all(&bindgen_dir).unwrap();
            generate_bindings(&bindgen_dir, msg_list);
            touch(&mark_file);
        } else {
            eprintln!("Used cached files in '{}'", bindgen_dir.display());
        }

        for filename in GENERATED_FILES {
            let src = bindgen_dir.join(filename);
            let tgt = out_dir.join(filename);
            fs::copy(&src, &tgt).unwrap();
        }

        #[cfg(feature = "save-bindgen")]
        {
            fs::create_dir_all(&save_dir).unwrap();

            for filename in GENERATED_FILES {
                let src = bindgen_dir.join(filename);
                let tgt = save_dir.join(filename);
                fs::copy(&src, &tgt).unwrap();
            }
        }
    }
}

fn generate_bindings(bindgen_dir: &Path, msg_list: &[RosMsg]) {
    let msg_includes_file = bindgen_dir.join(MSG_INCLUDES_FILENAME);
    let introspection_file = bindgen_dir.join(INTROSPECTION_FILENAME);
    let bindings_file = bindgen_dir.join(BINDINGS_FILENAME);

    let mut includes = String::new();
    let mut introspecion_map = String::from(
        "\
         lazy_static! {
           static ref INTROSPECTION_FNS: HashMap<&'static str, usize> = {
             let mut m = HashMap::new();
",
    );

    for msg in msg_list {
        // filename is certainly CamelCase -> snake_case. convert
        let include_filename = msg.name.to_snake_case();

        includes.push_str(&format!(
            "#include <{}/{}/{}.h>\n",
            &msg.module, &msg.prefix, &include_filename
        ));
        includes.push_str(&format!(
            "#include <{}/{}/detail/{}__rosidl_typesupport_introspection_c.h>\n",
            &msg.module, &msg.prefix, &include_filename
        ));

        if msg.prefix == "srv" {
            for s in &["Request", "Response"] {
                let key = &format!("{}__{}__{}_{}", &msg.module, &msg.prefix, &msg.name, s);
                let val = &format!("unsafe {{ rosidl_typesupport_introspection_c__get_message_type_support_handle__{}__{}__{}_{}() }} as *const i32 as usize", &msg.module, &msg.prefix, &msg.name, s);
                introspecion_map.push_str(&format!("m.insert(\"{}\", {});\n", key, val));
            }
        } else if msg.prefix == "action" {
            for s in &["Goal", "Result", "Feedback", "FeedbackMessage"] {
                let key = &format!("{}__{}__{}_{}", &msg.module, &msg.prefix, &msg.name, s);
                let val = &format!("unsafe {{ rosidl_typesupport_introspection_c__get_message_type_support_handle__{}__{}__{}_{}() }} as *const i32 as usize", &msg.module, &msg.prefix, &msg.name, s);
                introspecion_map.push_str(&format!("m.insert(\"{}\", {});\n", key, val));
            }
            // "internal" services
            for srv in &["SendGoal", "GetResult"] {
                // TODO: refactor this is copy paste from services...
                for s in &["Request", "Response"] {
                    let msgname = format!("{}_{}_{}", msg.name, srv, s);
                    let key = &format!("{}__{}__{}", &msg.module, &msg.prefix, msgname);
                    let val = &format!("unsafe {{ rosidl_typesupport_introspection_c__get_message_type_support_handle__{}__{}__{}() }} as *const i32 as usize", &msg.module, &msg.prefix, msgname);
                    introspecion_map.push_str(&format!("m.insert(\"{}\", {});\n", key, val));
                }
            }
        } else {
            let key = &format!("{}__{}__{}", &msg.module, &msg.prefix, &msg.name);
            let val = &format!("unsafe {{ rosidl_typesupport_introspection_c__get_message_type_support_handle__{}__{}__{}() }} as *const i32 as usize", &msg.module, &msg.prefix, &msg.name);
            introspecion_map.push_str(&format!("m.insert(\"{}\", {});\n", key, val));
        }
    }
    introspecion_map.push_str("m \n }; }\n\n");

    fs::write(&msg_includes_file, includes).unwrap();
    fs::write(&introspection_file, introspecion_map).unwrap();

    let builder = r2r_common::setup_bindgen_builder()
        .header(msg_includes_file.to_str().unwrap())
        .derive_copy(false)
        // blacklist types that are handled by rcl bindings
        .blocklist_type("rosidl_message_type_support_t")
        .blocklist_type("rosidl_service_type_support_t")
        .blocklist_type("rosidl_action_type_support_t")
        .blocklist_type("rosidl_runtime_c__String")
        .blocklist_type("rosidl_runtime_c__String__Sequence")
        .blocklist_type("rosidl_runtime_c__U16String")
        .blocklist_type("rosidl_runtime_c__U16String__Sequence")
        .blocklist_type("rosidl_runtime_c__float32__Sequence")
        .blocklist_type("rosidl_runtime_c__float__Sequence")
        .blocklist_type("rosidl_runtime_c__float64__Sequence")
        .blocklist_type("rosidl_runtime_c__double__Sequence")
        .blocklist_type("rosidl_runtime_c__long_double__Sequence")
        .blocklist_type("rosidl_runtime_c__char__Sequence")
        .blocklist_type("rosidl_runtime_c__wchar__Sequence")
        .blocklist_type("rosidl_runtime_c__boolean__Sequence")
        .blocklist_type("rosidl_runtime_c__octet__Sequence")
        .blocklist_type("rosidl_runtime_c__uint8__Sequence")
        .blocklist_type("rosidl_runtime_c__int8__Sequence")
        .blocklist_type("rosidl_runtime_c__uint16__Sequence")
        .blocklist_type("rosidl_runtime_c__int16__Sequence")
        .blocklist_type("rosidl_runtime_c__uint32__Sequence")
        .blocklist_type("rosidl_runtime_c__int32__Sequence")
        .blocklist_type("rosidl_runtime_c__uint64__Sequence")
        .blocklist_type("rosidl_runtime_c__int64__Sequence")
        .size_t_is_usize(true)
        .no_debug("_OSUnaligned.*")
        .generate_comments(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        });

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(bindings_file)
        .expect("Couldn't write bindings!");
}

fn run_dynlink(#[allow(unused_variables)] msg_list: &[RosMsg]) {
    #[cfg(not(feature = "doc-only"))]
    {
        r2r_common::print_cargo_link_search();

        let msg_map = r2r_common::as_map(msg_list);
        for module in msg_map.keys() {
            println!(
                "cargo:rustc-link-lib=dylib={}__rosidl_typesupport_c",
                module
            );
            println!(
                "cargo:rustc-link-lib=dylib={}__rosidl_typesupport_introspection_c",
                module
            );
            println!("cargo:rustc-link-lib=dylib={}__rosidl_generator_c", module);
        }
    }
}

fn touch(path: &Path) {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .unwrap_or_else(|_| panic!("Unable to create file '{}'", path.display()));
}
