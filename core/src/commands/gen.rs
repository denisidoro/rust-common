use crate::prelude::*;
use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct Input {}

impl Runnable for Input {
    fn run(&self, _system: System) -> Result<()> {
        println!("TODO");
        trace!("trace");
        info!("info");
        Ok(())
    }
}

impl HasDeps for Input {}
