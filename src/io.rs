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
        "hl.monitor({{ output = \"{}\", refresh = {} }})",
        disp.name, refresh_rate
    );

    hypr(&["eval", &code]);
}

pub fn scale(disp: &Disp, scale: f32) {
    let code = format!(
        "hl.monitor({{ output = \"{}\", scale = {} }})",
        disp.name, scale
    );
    hypr(&["eval", &code]);
}