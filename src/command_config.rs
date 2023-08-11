use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use std::path::PathBuf;

pub type Flag = String;
pub type Value = serde_yaml::Value;
pub type FlagMap = BTreeMap<Flag, Vec<Value>>;

#[derive(Serialize, Deserialize, Debug)]
struct RawConfig {
    flags: BTreeMap<Flag, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandConfig {
    pub flags: FlagMap,
}

impl CommandConfig {
    fn from_raw(raw_config: RawConfig) -> CommandConfig {
        fn transform_value(value: &Value) -> Vec<Value> {
            if value.is_sequence() {
                value
                    .as_sequence()
                    .expect("It should be a sequence.")
                    .to_vec()
            } else {
                vec![value.clone()]
            }
        }

        fn transform_flags(flags: &BTreeMap<Flag, Value>) -> FlagMap {
            flags
                .iter()
                .map(|(key, value)| (key.clone(), transform_value(value)))
                .collect()
        }

        CommandConfig {
            flags: transform_flags(&raw_config.flags),
        }
    }

    pub fn from_path(path: PathBuf) -> Result<CommandConfig, serde_yaml::Error> {
        let buf_reader = BufReader::new(File::open(path).expect("Can't open file."));
        serde_yaml::from_reader(buf_reader).map(Self::from_raw)
    }

    //#![allow(dead_code)]
    pub fn from_string(raw: String) -> Result<CommandConfig, serde_yaml::Error> {
        serde_yaml::from_str(raw.as_str().deref()).map(Self::from_raw)
    }
}
