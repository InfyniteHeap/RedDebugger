use crate::{LitematicModuleData, NbtModuleData, SchemModuleData};

use std::io::Read;

use fastnbt::from_bytes;
use flate2::read::GzDecoder;
use fs_err as fs;

pub fn parse_nbt_file(nbt: fs::File) -> anyhow::Result<()> {
    let mut decoder = GzDecoder::new(nbt);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    let result = from_bytes::<NbtModuleData>(data.as_slice())?;
    println!("{:?}", result);

    Ok(())
}

pub fn parse_schem_file(schem: fs::File) -> anyhow::Result<()> {
    let mut decoder = GzDecoder::new(schem);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    let result = from_bytes::<SchemModuleData>(data.as_slice())?;
    println!("{:?}", result);

    Ok(())
}

pub fn parse_litematic_file(litematic: fs::File) -> anyhow::Result<()> {
    let mut decoder = GzDecoder::new(litematic);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    let result = from_bytes::<LitematicModuleData>(data.as_slice())?;
    println!("{:?}", result);

    Ok(())
}
