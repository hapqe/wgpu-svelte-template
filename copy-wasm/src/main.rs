use std::{fs, io, path::Path};

const WASM_INPUT: &str = "../pkg";
const WASM_OUTPUT: &str = "../site/public/";

fn main() {
    copy_wasm().expect("Failed to copy wasm");
    // fix_snippets().expect("Failed to fix snippets");
}

fn copy_wasm() -> Result<(), io::Error> {
    let paths = fs::read_dir(WASM_INPUT)?;

    for entry in paths {
        let path = entry?.path();
        if let Some(extension) = Path::new(&path).extension() {
            if extension.to_ascii_lowercase() == "wasm" {
                if let Some(name) = path.file_name() {
                    if let Some(name) = name.to_str() {
                        let from = Path::new(WASM_INPUT).join(name);
                        let to = Path::new(&WASM_OUTPUT).join(name);
                        println!("Copying {}", to.to_str().unwrap());

                        fs::copy(&from, &to)?;
                    }
                }
            }
        }
    }

    Ok(())
}
