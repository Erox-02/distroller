use anyhow::Result;
use serde::Deserialize;
use std::process::Command;

pub struct Disp {
    pub name: String,
    pub refresh_rate: f32,
    pub w: u16,
    pub h: u16,
    pub active: bool,
}

#[derive(Deserialize)]
struct Moniter {
    name: String,
    refresh_rate: f32,
    w: u16,
    h: u16,
    active: bool,
}


fn get_displays() -> Result<Vec<Disp>> {
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("hyprctl err");
    }

    let monitors: Vec<HyprMonitor> =
        serde_json::from_slice(&output.stdout)?;

    let displays = monitors
        .into_iter()
        .map(|m| Disp {
            name: m.name,
            refresh_rate: m.refresh_rate,
            w: m.w,
            h: m.h,
            active: m.active,
        })
        .collect();

    Ok(displays)
}