mod module;
mod normalize_module;
mod parse_module;

pub use module::{litematic::LitematicModuleData, nbt::NbtModuleData, schem::SchemModuleData};
pub use normalize_module::UnifiedBlock;

use parse_module::{parse_litematic_file, parse_nbt_file, parse_schem_file};

use std::fmt::Debug;

use fs_err as fs;

pub trait Module: Debug {}

fn main() -> anyhow::Result<()> {
    loop {
        println!("Please enter the path of a module.");
        println!("Only NBT format, SCHEM format and LITEMATIC format are accepted!");

        let mut path = String::new();
        std::io::stdin().read_line(&mut path)?;
        let file_path = path.trim().parse::<String>()?;

        let path = file_path.clone();
        let suffix = path.split_inclusive('.').collect::<Vec<&str>>();

        if suffix.is_empty() {
            eprintln!("Nothing inputted in console!");
            continue;
        } else {
            let module_file = match fs::File::open(file_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{e}");
                    continue;
                }
            };

            match suffix[suffix.len() - 1] {
                "nbt" => parse_nbt_file(module_file)?,
                "schem" => parse_schem_file(module_file)?,
                "litematic" => parse_litematic_file(module_file)?,
                _ => {
                    eprintln!("Unsupported file!");
                    continue;
                }
            }

            break;
        }
    }

    println!("Press enter to close this window.");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    Ok(())
}
