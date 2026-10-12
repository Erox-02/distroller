use crate::ls::{hypr, Disp};

pub fn set_disp(disp: &Disp, disabled: bool) {
    let code = format!(
        "hl.monitor({{ output = \"{}\", disabled = {} }})",
        disp.name, disabled
    );
    hypr(&["eval", &code]);
}

pub fn enable(disp: &Disp) {
    set_disp(disp, false);
}

pub fn disable(disp: &Disp) {
    set_disp(disp, true);
}


pub fn refresh(disp: &Disp, refresh_rate: f32) {
    let code = format!(
        "hl.monitor({{ output = \"{}\", mode = \"{}x{}@{}\" }})",
        disp.name, disp.width, disp.height, refresh_rate
    );

    println!("Running: {code}");
    hypr(&["eval", &code]);
}


pub fn scale(disp: &Disp, scale: f32) {
    let code = format!(
        "hl.monitor({{ output = \"{}\", scale = {} }})",
        disp.name, scale
    );
    hypr(&["eval", &code]);
}