use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use anyhow::{Context, Result};
use std::collections::VecDeque;

#[derive(StructOpt, Debug)]
#[structopt(name = "saucy")]
/// saucy is a basic script for finding python virtual environments.
struct Opt {
    #[structopt(name = "PATH", default_value = ".", parse(from_os_str))]
    /// The path of the directory to search for a virtualenv.
    path: PathBuf,

    #[structopt(short, long)]
    /// By default saucy returns the first python environment it finds.  If the `find-all` flag
    /// is enabled, then saucy will print out all of the virtual environments it finds (instead of
    /// just the first).
    find_all : bool,
}

#[cfg(target_os = "windows")]
const VENV_BIN: &str = "Scripts";
#[cfg(not(target_os = "windows"))]
const VENV_BIN: &str = "bin";

#[cfg(target_os = "windows")]
const VENV_PYBIN: &str = "python.exe";
#[cfg(not(target_os = "windows"))]
const VENV_PYBIN: &str = "python";

fn get_virtual_env<P: AsRef<Path>>(p: P) -> Option<PathBuf> {
    // Is there a virtualenv in path p?
    let mut p = p.as_ref().to_path_buf();

    // Must be a directory
    if !p.is_dir() {
        return None;
    }

    // Does $p/VENV_BIN exist?
    p.push(VENV_BIN);
    if !p.exists() || !p.is_dir() {
        return None;
    }

    // Does $p/VENV_PYBIN/python exist?
    p.push(VENV_PYBIN);
    if !p.exists() || !p.is_file() {
        return None;
    }

    // Return $p/VENV_PYBIN/activate
    p.pop();
    p.push("activate");

    Some(p)
}

fn display_virtual_env<P: AsRef<Path>>(p: P) -> Result<()> {
    let p = p.as_ref();

    let p = dunce::realpath(p).with_context(|| {
        format!(
            "unable to get canonical path for relative path {}",
            p.display()
        )
    })?;

    println!("{}", p.display());

    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_front(opt.path);

    while let Some(entry) = queue.pop_front() {
        // Check whether entry is a virtual env?
        if let Some(path) = get_virtual_env(&entry) {
            display_virtual_env(path)?;

            if !opt.find_all {
                return Ok(());
            }
        }

        // Otherwise extend the queue with all subdirectories of this entry
        if let Ok(directory) = fs::read_dir(&entry) {
            for file in directory
                {
                    let file = file?;
                    let path = file.path();

                    if path.is_dir() {
                        queue.push_back(path);
                    }
                }
        }
    }

    Ok(())
}
