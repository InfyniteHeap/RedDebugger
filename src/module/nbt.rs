use crate::Module;

use std::{borrow::Cow, collections::HashMap};

use red_runtime::RedstoneComponent;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NbtModuleData<'a> {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    pub size: [i32; 3],
    pub palette: Vec<Palette<'a>>,
    pub blocks: Vec<Block<'a>>,
}

impl Module for NbtModuleData<'_> {}
impl RedstoneComponent for NbtModuleData<'_> {}

#[derive(Deserialize, Debug)]
pub struct Palette<'a> {
    #[serde(rename = "Name")]
    pub name: Cow<'a, str>,
    #[serde(rename = "Properties")]
    pub properties: Option<Properties<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct Properties<'a>(HashMap<Cow<'a, str>, Cow<'a, str>>);

#[derive(Deserialize, Debug)]
pub struct Block<'a> {
    pub state: i32,
    pub pos: [i32; 3],
    pub nbt: Option<Nbt<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct Nbt<'a> {
    #[serde(rename = "Items")]
    pub items: Option<Vec<Items<'a>>>,
    pub id: Option<Cow<'a, str>>,
    #[serde(rename = "OutputSignal")]
    pub output_signal: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Items<'a> {
    #[serde(rename = "Slot")]
    pub slot: i8,
    pub id: Cow<'a, str>,
    #[serde(rename = "Count")]
    pub count: i8,
}
