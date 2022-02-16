use crate::hook;
use failure::Error;
use std::path::PathBuf;
use std::vec::Vec;
use std::process::Command;

/// Execute the provided command (argv) after loading the environment from the current directory
pub fn run(pathbuf: PathBuf, shadowenv_data: String, argv: Vec<&str>) -> Result<(), Error> {
    match hook::load_env(pathbuf, shadowenv_data, true)? {
        Some((shadowenv, _)) => {
            hook::mutate_own_env(&shadowenv)?;
        }
        None => (),
    }

    if let Err(err) = Command::new(&argv[0]).args(&argv[1..]).spawn()?.wait() {
        return Err(err.into())
    }
    Ok(())
}
