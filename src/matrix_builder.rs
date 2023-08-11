use crate::command_config::{CommandConfig, Flag, Value};
use std::collections::BTreeMap;
use std::ops::Deref;
use std::path::PathBuf;

pub type FlagMap = BTreeMap<Flag, Vec<Value>>;

pub struct MatrixBuilder {}

impl MatrixBuilder {
    pub fn from_path(path: PathBuf) -> Vec<BTreeMap<Flag, Value>> {
        let command_config = CommandConfig::from_path(path).expect("Failed parsing.");
        Self::generate_commands(command_config)
    }

    fn generate_commands(command_config: CommandConfig) -> Vec<BTreeMap<Flag, Value>> {
        fn permute<'a>(flags: &'a [&Flag], flag_map: &'a FlagMap) -> Vec<BTreeMap<Flag, Value>> {
            match flags {
                [] => vec![BTreeMap::new()],
                [flag, tail @ ..] => {
                    let values = permute(tail, flag_map);
                    flag_map
                        .get(flag.deref().clone().as_str())
                        .unwrap()
                        .iter()
                        .flat_map(|pre| {
                            values.iter().map(|previous| {
                                let mut map = BTreeMap::from_iter(
                                    previous.iter().map(|(k, v)| (k.clone(), v.clone())),
                                );
                                map.insert(flag.deref().clone(), pre.clone());
                                map
                            })
                        })
                        .collect()
                }
            }
        }

        let flag_keys: Vec<&Flag> = command_config.flags.keys().collect();
        permute(&flag_keys, &command_config.flags)
    }
}
