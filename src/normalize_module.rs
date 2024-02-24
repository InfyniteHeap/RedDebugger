use crate::{LitematicModuleData, NbtModuleData, SchemModuleData};

use fastnbt::Value;
use red_runtime::UnifiedBlock;

pub fn normalize_from_nbt(nbt: NbtModuleData) -> Vec<UnifiedBlock> {
    todo!()
}

pub fn normalize_from_schem(schem: SchemModuleData) -> Vec<UnifiedBlock> {
    let mut schem_vec = Vec::new();

    for val in schem.block_entities {
        let mut schem = UnifiedBlock::default();

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

pub fn normalize_from_litematic(litematic: LitematicModuleData) -> Vec<UnifiedBlock> {
    todo!()
}
