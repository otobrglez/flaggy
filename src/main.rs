use crate::command_config::{Flag, Value};
use crate::matrix_builder::MatrixBuilder;
use clap::Parser;
use std::collections::BTreeMap;
use std::path::PathBuf;

mod command_config;
mod matrix_builder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FLAGS_FILE")]
    flags_file: PathBuf,

    #[arg(short, long, default_value = ";")]
    separator: String,

    #[arg(long, default_value = "=")]
    assigment: String,

    #[arg(long, default_value = "\n")]
    newline: String,
}

fn flagmap_to_line(
    flag_map: BTreeMap<Flag, Value>,
    assigment: &String,
    separator: &String,
) -> String {
    flag_map
        .iter()
        .map(|(k, v)| {
            format!(
                "{}{}{}",
                k.clone(),
                assigment,
                serde_yaml::to_string(v).unwrap().trim()
            )
        })
        .collect::<Vec<String>>()
        .join(separator)
}

fn main() {
    let cli = Cli::parse();
    for flagmap in MatrixBuilder::from_path(cli.flags_file) {
        print!(
            "{}{}",
            flagmap_to_line(flagmap, &cli.assigment, &cli.separator),
            cli.newline
        );
    }
}
