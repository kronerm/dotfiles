use std::{
    collections::BTreeMap,
    io::Write as _,
    process::{Command, Stdio},
};

use itertools::Itertools as _;
use serde::Deserialize;

#[derive(Deserialize)]
struct PhysicalSize {
    width: usize,
    height: usize,
}

#[derive(Deserialize)]
struct Mode {
    width: usize,
    height: usize,
    refresh: f64,
    preferred: bool,
    current: bool,
}

#[derive(Deserialize)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Transform {
    Normal,
    #[serde(rename = "90")]
    Deg90,
    #[serde(rename = "180")]
    Deg180,
    #[serde(rename = "270")]
    Deg270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

#[derive(Deserialize)]
struct Display {
    name: Option<String>,
    description: Option<String>,
    make: Option<String>,
    model: Option<String>,
    serial: Option<String>,
    physical_size: PhysicalSize,
    enabled: bool,
    modes: Vec<Mode>,
    position: Position,
    transform: Transform,
    scale: f64,
    adaptive_sync: bool,
}

fn select_with_rofi(items: &[String]) -> eyre::Result<String> {
    let mut child = Command::new("rofi")
        .args(["-dmenu"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    std::thread::scope(|scope| {
        scope.spawn(move || {
            stdin.write_all(items.join("\n").as_bytes()).unwrap();
            stdin.flush().unwrap();
        });
    });

    Ok(String::from_utf8(child.wait_with_output()?.stdout)?
        .trim()
        .to_string())
}

pub struct DesiredMode {
    pub width: usize,
    pub height: usize,
    pub refresh_rate: f64,
}

pub fn get_desired_mode() -> eyre::Result<DesiredMode> {
    let wlr_randr_output: Vec<Display> =
        serde_json::from_slice(&Command::new("wlr-randr").args(["--json"]).output()?.stdout)?;

    let modes = wlr_randr_output
        .into_iter()
        .flat_map(|display| display.modes)
        .collect::<Vec<_>>();

    let chunks = modes
        .chunk_by(|m1, m2| m1.width == m2.width && m1.height == m2.height)
        .map(|modes_chunk| {
            (
                (modes_chunk[0].width, modes_chunk[0].height),
                modes_chunk
                    .iter()
                    .map(|mode| mode.refresh)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let resolutions = chunks
        .keys()
        .sorted()
        .rev()
        .map(|(width, height)| format!("{width}x{height}"))
        .collect::<Vec<_>>();
    let resolution = select_with_rofi(&resolutions)?;
    let split = resolution
        .split('x')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (width, height) = (split[0], split[1]);

    let refresh_rates = chunks[&(width, height)]
        .iter()
        .map(|rate| rate.to_string())
        .collect::<Vec<_>>();
    let refresh_rate = select_with_rofi(&refresh_rates)?.parse::<f64>()?;

    Ok(DesiredMode {
        width,
        height,
        refresh_rate,
    })
}
