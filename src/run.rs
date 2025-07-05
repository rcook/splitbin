use crate::args::{Args, Command, EndOrLen};
use crate::util::open_for_write;
use anyhow::{Result, anyhow};
use clap::Parser;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub fn run() -> Result<()> {
    match Args::parse().command {
        Command::Chunks {
            path,
            chunk_size,
            overwrite,
        } => chunks(&path, chunk_size, overwrite),
        Command::Extract {
            path,
            output_path,
            start,
            end_or_len,
            overwrite,
        } => extract(&path, &output_path, start, end_or_len.as_ref(), overwrite),
    }
}

pub fn chunks(path: &Path, chunk_size: usize, overwrite: bool) -> Result<()> {
    let d = path.parent().ok_or_else(|| {
        anyhow!(
            "could not get parent directory from path {path}",
            path = path.display()
        )
    })?;
    let file_name = path.file_name().and_then(OsStr::to_str).ok_or_else(|| {
        anyhow!(
            "could not get file name from path {path}",
            path = path.display()
        )
    })?;

    let mut f = File::open(path)?;
    let mut len = usize::try_from(f.seek(SeekFrom::End(0))?).unwrap();
    _ = f.seek(SeekFrom::Start(0));

    let mut bytes = vec![0; chunk_size];
    for i in 0.. {
        if len == 0 {
            break;
        }

        let output_path = d.join(format!("{file_name}-{i:02}"));
        let count = chunk_size.min(len);
        f.read_exact(&mut bytes[0..count])?;
        let mut output_f = open_for_write(&output_path, overwrite)?;
        output_f.write_all(&bytes[..count])?;
        len -= count;
    }

    Ok(())
}

pub fn extract(
    path: &Path,
    output_path: &Path,
    start: usize,
    end_or_len: Option<&EndOrLen>,
    overwrite: bool,
) -> Result<()> {
    let mut f = File::open(path)?;
    let total_len = usize::try_from(f.seek(SeekFrom::End(0))?).unwrap();
    let remaining = total_len - start;
    let len = match end_or_len {
        Some(EndOrLen::End(end)) => total_len.min(end - start),
        Some(EndOrLen::Len(len)) => remaining.min(*len),
        None => remaining,
    };

    let mut bytes = vec![0; len];
    _ = f.seek(SeekFrom::Start(u64::try_from(start)?));
    f.read_exact(&mut bytes)?;

    let mut output_f = open_for_write(output_path, overwrite)?;
    output_f.write_all(&bytes)?;

    Ok(())
}
