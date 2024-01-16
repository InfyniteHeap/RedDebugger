pub mod litematic_data;
pub mod nbt_data;
pub mod schem_data;

pub use self::litematic_data::LitematicModuleData;
pub use self::nbt_data::NbtModuleData;
pub use self::schem_data::SchemModuleData;

use std::io::Read;

use fastnbt::from_bytes;
use flate2::read::GzDecoder;
use fs_err as fs;

pub fn open_module_file(file_path: &str) -> anyhow::Result<(fs::File, i32)> {
    let suffix = file_path.split_inclusive('.').collect::<Vec<&str>>();

    if suffix.is_empty() {
        Err(anyhow::Error::msg("Nothing inputted in console!"))
    } else {
        match suffix[suffix.len() - 1] {
            "nbt" => Ok((fs::File::open(file_path)?, 0)),
            "schem" => Ok((fs::File::open(file_path)?, 1)),
            "litematic" => Ok((fs::File::open(file_path)?, 2)),
            _ => Err(anyhow::Error::msg("Invalid file!")),
        }
    }
}

pub fn parse_nbt_file(nbt: fs::File) -> anyhow::Result<nbt_data::NbtModuleData<'static>> {
    let mut decoder = GzDecoder::new(nbt);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    Ok(from_bytes::<nbt_data::NbtModuleData>(data.as_slice())?)
}

pub fn parse_schem_file(schem: fs::File) -> anyhow::Result<schem_data::SchemModuleData<'static>> {
    let mut decoder = GzDecoder::new(schem);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    Ok(from_bytes::<schem_data::SchemModuleData>(data.as_slice())?)
}

pub fn parse_litematic_file(
    litematic: fs::File,
) -> anyhow::Result<litematic_data::LitematicModuleData<'static>> {
    let mut decoder = GzDecoder::new(litematic);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    Ok(from_bytes::<litematic_data::LitematicModuleData>(
        data.as_slice(),
    )?)
}
