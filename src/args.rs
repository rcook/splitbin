use clap::{Parser, Subcommand};
use clap_num::maybe_hex;
use path_absolutize::Absolutize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(name = "chunks", about = "Split binary file into chunks")]
    Chunks {
        #[arg(value_parser = parse_absolute_path)]
        path: PathBuf,

        #[arg(value_parser = maybe_hex::<usize>)]
        chunk_size: usize,

        #[arg(
            help = "Overwrite output file if it already exists",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,
    },

    #[command(name = "extract", about = "Extract section of file")]
    Extract {
        #[arg(value_parser = parse_absolute_path)]
        path: PathBuf,

        #[arg(value_parser = parse_absolute_path)]
        output_path: PathBuf,

        #[arg(value_parser = maybe_hex::<usize>)]
        start: usize,

        #[arg(
            help = "End of section or prefix with \"+\" to specify length",
            value_parser = parse_end_or_len
        )]
        end_or_len: Option<EndOrLen>,

        #[arg(
            help = "Overwrite output file if it already exists",
            long = "overwrite",
            short = 'f',
            default_value_t = false
        )]
        overwrite: bool,
    },
}

#[derive(Clone, Debug)]
pub enum EndOrLen {
    End(usize),
    Len(usize),
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}

fn parse_end_or_len(s: &str) -> Result<EndOrLen, String> {
    match s.strip_prefix('+') {
        Some(s) => Ok(EndOrLen::Len(maybe_hex(s)?)),
        None => Ok(EndOrLen::End(maybe_hex(s)?)),
    }
}
