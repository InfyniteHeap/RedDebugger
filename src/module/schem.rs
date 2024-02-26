use std::{borrow::Cow, collections::HashMap};

use fastnbt::{ByteArray, IntArray, Value};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SchemModuleData<'a> {
    #[serde(rename = "Version")]
    pub version: i32,
    #[serde(rename = "DataVersion")]
    pub data_version: i32,

    #[serde(rename = "Length")]
    pub length: i16,
    #[serde(rename = "Width")]
    pub width: i16,
    #[serde(rename = "Height")]
    pub height: i16,

    #[serde(rename = "Metadata")]
    pub meta_data: MetaData,
    #[serde(rename = "Offset")]
    pub offset: IntArray,
    #[serde(rename = "PaletteMax")]
    pub palette_max: i32,
    #[serde(rename = "Palette")]
    pub palette: Palette<'a>,

    #[serde(rename = "BlockData")]
    pub block_data: ByteArray,
    // As each element (structure) in "BlockEntities" contains uncertain-number fields,
    // "Value" has to be used there.
    #[serde(rename = "BlockEntities")]
    pub block_entities: Vec<Value>,
}

#[derive(Deserialize)]
pub struct MetaData {
    #[serde(rename = "WEOffsetX")]
    pub we_offset_x: i32,
    #[serde(rename = "WEOffsetY")]
    pub we_offset_y: i32,
    #[serde(rename = "WEOffsetZ")]
    pub we_offset_z: i32,
}

#[derive(Deserialize)]
pub struct Palette<'a>(HashMap<Cow<'a, str>, i32>);

// #[derive(Deserialize)]
// pub struct Entity<'a> {
//     #[serde(rename = "Id")]
//     pub id: Cow<'a, str>,
//     #[serde(rename = "Items")]
//     pub item: Vec<Items>,
//     #[serde(rename = "Pos")]
//     pub pos: [i32; 3],

//     pub burn_time: Option<i16>,
//     pub cook_time: Option<i16>,
//     pub cook_time_total: Option<i16>,
//     #[serde(rename = "CustomName")]
//     pub custom_name: Option<Cow<'a, str>>,
//     pub output_signal: i32,
// }

// #[derive(Deserialize, Debug)]
// pub struct Items {}
