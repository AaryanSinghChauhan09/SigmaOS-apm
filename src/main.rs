//! SigmaOS-apm - SigmaOS Component

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting SigmaOS-apm");
    println!("SigmaOS SigmaOS-apm v0.1.0");
    Ok(())
}
