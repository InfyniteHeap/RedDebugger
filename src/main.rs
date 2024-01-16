mod normalize_module;
mod parse_module;

use crate::parse_module::*;

use std::fmt::Debug;

use fs_err as fs;

pub trait Module: Debug {}

fn main() -> anyhow::Result<()> {
    let module_file: fs::File;
    let num: i32;

    loop {
        println!("Please enter the path of a module.");
        println!("Only NBT format, SCHEM format and LITEMATIC format are accepted!");

        let mut path = String::new();
        std::io::stdin().read_line(&mut path)?;
        let path = path.trim().parse::<String>()?;

        match open_module_file(&path) {
            Ok(file) => {
                (module_file, num) = file;
                break;
            }
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        }
    }

    let parse_result = parse_module(module_file, num)?;
    println!("{:#?}", parse_result);

    println!("Press enter to close this window.");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    Ok(())
}

fn parse_module(module_file: fs::File, num: i32) -> anyhow::Result<Box<dyn Module>> {
    match num {
        0 => Ok(Box::new(parse_nbt_file(module_file)?)),
        1 => Ok(Box::new(parse_schem_file(module_file)?)),
        2 => Ok(Box::new(parse_litematic_file(module_file)?)),
        _ => unreachable!(),
    }
}
