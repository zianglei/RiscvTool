use std::path::Path;
use std::collections::HashMap;
use std::fs;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use serde::{Serialize};

#[derive(Serialize)]
pub struct InstArgs {
    q: String,
    l: String,
    rs1 : String,
    rs2: String,
    rd: String,
    rm: String
}

#[derive(Serialize)]
pub struct InstMeta {
    encoding: String
}

#[derive(Serialize)]
pub struct Inst {
    mask: String,
    args: InstArgs,
    meta: InstMeta
}

impl Inst {
    fn new(mask: String, args: InstArgs, meta: InstMeta) -> Inst {
        return Inst { mask, args, meta};
    }
}

pub type IsaMapType = HashMap<String, HashMap::<String, Inst>>;

pub fn parse_isa(isa_dir: &str, isa_name: &str) -> IsaMapType {
    let isa_path = Path::new(isa_dir).join(isa_name);
    let mut inst_list = IsaMapType::new();

    for entry in isa_path.read_dir().expect("read_dir failed") {
        if let Ok(entry) = entry {
            let mut type_inst_map = HashMap::<String, Inst>::new();
            for inst_file_path in entry.path().read_dir().expect("isa type folder read_dir failed") {
                if let Ok(inst_file_path) = inst_file_path {
                    let inst_yml_file: String = fs::read_to_string(inst_file_path.path()).expect("read_to_string failed");
                    let inst_yml = &YamlLoader::load_from_str(&inst_yml_file).unwrap()[0];
                    let args = InstArgs {
                        q   : inst_yml["args"]["q"].as_str().unwrap_or_default().to_string(),
                        rs1 : inst_yml["args"]["rs1"].as_str().unwrap_or_default().to_string(),
                        rs2 : inst_yml["args"]["rs2"].as_str().unwrap_or_default().to_string(),
                        l   : inst_yml["args"]["l"].as_str().unwrap_or_default().to_string(),
                        rd  : inst_yml["args"]["rd"].as_str().unwrap_or_default().to_string(),
                        rm  : inst_yml["args"]["rm"].as_str().unwrap_or_default().to_string()
                    };
                    let meta = InstMeta {
                        encoding: inst_yml["args"]["q"].as_str().unwrap_or_default().to_string(),
                    };
                    let mask = inst_yml["mask"].as_str().unwrap_or_default().to_string();

                    let inst = Inst::new(mask, args, meta);
                    type_inst_map.insert(inst_file_path.path().file_stem().unwrap().
                        to_os_string().into_string().unwrap(), inst);
                }
            }
            inst_list.insert(entry.file_name().into_string().unwrap(), type_inst_map);
        }
    }
    inst_list
}