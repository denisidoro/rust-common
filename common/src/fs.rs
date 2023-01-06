use crate::prelude::*;
use directories_next::BaseDirs;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub trait ToStringExt {
    fn to_string(&self) -> String;
}

impl ToStringExt for Path {
    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

impl ToStringExt for OsStr {
    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

pub fn config_dir(project_name: &str) -> Result<PathBuf> {
    let base_dirs = BaseDirs::new().context("unable to get base dirs")?;

    let mut pathbuf = PathBuf::from(base_dirs.config_dir());
    pathbuf.push(project_name);
    Ok(pathbuf)
}

pub fn open(filename: &Path) -> Result<File> {
    File::open(filename).with_context(|| {
        let x = filename.to_string();
        format!("Failed to open file {}", &x)
    })
}

pub fn read_lines(filename: &Path) -> Result<impl Iterator<Item = String>> {
    let file = open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.map(|x| x.expect("bad line")))
}

pub fn remove_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    std::fs::remove_file(path).with_context(|| format!("Failed to rm {}", path.to_string()))?;
    Ok(())
}
