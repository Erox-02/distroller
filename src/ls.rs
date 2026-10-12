use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Disp {
    pub id: u8,
    pub name: String,
    pub refreshRate: f32,
    pub width: u16,
    pub height: u16,
    pub focused: bool,
    pub disabled: bool,
    #[serde(rename = "availableModes")]
    pub available_modes: Vec<String>,
}


pub fn hypr(args: &[&str]) -> String {
    let output = Command::new("hyprctl")
        .args(args)
        .output()
        .expect("hyprctl's err not mine");

    String::from_utf8(output.stdout)
        .expect("hyprctl returned invalid utf8")
}

pub fn get_disp() -> Vec<Disp> {
    let output = hypr(&["monitors", "all", "-j"]);

    serde_json::from_str(&output)
        .expect("cant parse monitor data")
}