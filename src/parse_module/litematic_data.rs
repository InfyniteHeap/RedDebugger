use crate::Module;

use std::borrow::Cow;

use fastnbt::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LitematicModuleData<'a> {
    #[serde(rename = "Version")]
    pub version: i32,
    #[serde(rename = "SubVersion")]
    pub sub_version: i32,
    #[serde(rename = "MinecraftDataVersion")]
    pub minecraft_data_version: i32,

    #[serde(rename = "Metadata")]
    pub metadata: MetaData<'a>,
    // As the name of the structure depends on "Name" in "Metadata",
    // "Value" has to be used there.
    #[serde(rename = "Regions")]
    pub regions: Value,
}

impl Module for LitematicModuleData<'_> {}

#[derive(Deserialize, Debug)]
pub struct MetaData<'a> {
    #[serde(rename = "Author")]
    pub author: Cow<'a, str>,
    #[serde(rename = "Name")]
    pub name: Cow<'a, str>,
    #[serde(rename = "Description")]
    pub description: Cow<'a, str>,

    #[serde(rename = "RegionCount")]
    pub region_count: i32,
    #[serde(rename = "TotalBlocks")]
    pub total_blocks: i32,
    #[serde(rename = "TotalVolume")]
    pub total_volume: i32,

    #[serde(rename = "TimeCreated")]
    pub time_created: i64,
    #[serde(rename = "TimeModified")]
    pub time_modified: i64,

    #[serde(rename = "EnclosingSize")]
    pub enclosing_size: Size,
}

#[derive(Deserialize, Debug)]
pub struct Size {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
