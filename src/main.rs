use std::process::Command;
use serde::Deserialize;

pub struct Disp {
    pub name: String,
    pub refreshRate: f32,
    pub w: u16,
    pub h: u16,
    pub focused: bool,
    pub disabled: bool,
}

pub fn hypr(args: &[&str]) -> String {
    let output = Command::new("hyprctl")
        .args(args)
        .output()
        .expect("hyprctl's err not mine ");

    String::from_utf8(output.stdout)
        .expect("hyprctl returned invalid utf8")
}

pub fn get_disp() -> Vec<Disp> {
    let output = hypr(&["monitors", "-j"]);

    serde_json::from_str(&output)
        .expect("cant parse monitor data")
}