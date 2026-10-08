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