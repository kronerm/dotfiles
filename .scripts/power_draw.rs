#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! anyhow = "1"
//! ```

use std::fs;

use anyhow::Context as _;

const BATTERY_DIR: &str = "/sys/class/power_supply/BAT0";

fn f64_from_file(path: &str) -> anyhow::Result<f64> {
    fs::read_to_string(path)?
        .trim()
        .parse::<f64>()
        .context("failed to parse")
}

fn main() -> anyhow::Result<()> {
    let current_now = f64_from_file(&format!("{BATTERY_DIR}/current_now"))?;
    let voltage_now = f64_from_file(&format!("{BATTERY_DIR}/voltage_now"))?;
    print!("{:.1}", (current_now * voltage_now) / 1000000000000.);

    Ok(())
}
