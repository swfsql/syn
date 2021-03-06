use proc_macro2::TokenStream;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write<P: AsRef<Path>>(path: P, content: TokenStream) {
    let mut file = File::create(path).unwrap();
    write!(
        file,
        "// THIS FILE IS AUTOMATICALLY GENERATED; DO NOT EDIT\n\n"
    )
    .unwrap();
    let mut config = rustfmt::Config::default();
    config.set().emit_mode(rustfmt::EmitMode::Stdout);
    config.set().verbose(rustfmt::Verbosity::Quiet);
    config.set().format_macro_matchers(true);
    config.set().normalize_doc_attributes(true);
    let mut session = rustfmt::Session::new(config, Some(&mut file));
    session
        .format(rustfmt::Input::Text(content.to_string()))
        .unwrap();
}
