use crate::{LitematicModuleData, NbtModuleData, SchemModuleData};

use fastnbt::Value;

#[derive(Default)]
pub struct UnifiedBlock {
    pub name: String,
    pub id: String,
    pub pos: Vec<i32>,
    pub signal: i32,
}

impl UnifiedBlock {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            pos: Vec::new(),
            signal: 0,
        }
    }

    pub fn normalize_from_nbt(&mut self, nbt: NbtModuleData) -> Vec<Self> {
        todo!()
    }

    pub fn normalize_from_schem(&mut self, schem: SchemModuleData) -> Vec<Self> {
        let mut schem_vec = Vec::new();

        for val in schem.block_entities {
            let mut schem = Self::new();

            if let Value::List(ls) = val {
                for val in ls {
                    if let Value::Compound(cp) = val {
                        for (str, val) in cp {
                            match str.as_str() {
                                "Id" => {
                                    if let Value::String(str) = val {
                                        schem.id = str;
                                        schem.name = schem.id[10..].to_owned();
                                    }
                                }
                                "Pos" => {
                                    if let Value::IntArray(arr) = val {
                                        schem.pos = arr.into_inner();
                                    }
                                }
                                "OutputSignal" => {
                                    if let Value::Int(int) = val {
                                        schem.signal = int;
                                    }
                                }
                                _ => continue,
                            }
                        }
                    }
                }
            }

            schem_vec.push(schem)
        }

        schem_vec
    }

    pub fn normalize_from_litematic(&mut self, litematic: LitematicModuleData) -> Vec<Self> {
        todo!()
    }
}
