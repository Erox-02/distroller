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

    println!("running => {code}");
    hypr(&["eval", &code]);
}


pub fn scale(disp: &Disp, scale: f32) {
    let code = format!(
        "hl.monitor({{ output = \"{}\", scale = {} }})",
        disp.name, scale
    );
    hypr(&["eval", &code]);
}


pub fn position(disp: &Disp, q: i32, r: i32) {
    let code = format!(
        "hl.monitor({{ output = \"{}\", position = \"{}x{}\", scale = {} }})",
        disp.name, q, r, disp.scale
    );
    hypr(&["eval", &code]);
}

pub fn sup_refresh(disp: &Disp, refresh_rate: f32) -> bool {
    if !refresh_rate.is_finite() || refresh_rate <= 0.0 {
        return false;
    }
    let resolution = format!("{}x{}", disp.width, disp.height);

    disp.available.iter().any(|mode| {
        let Some((mode_resolution, rate)) = mode.split_once('@') else {
            return false;
        };

        let Some(rate) = rate
            .strip_suffix("Hz")
            .and_then(|rate| rate.parse::<f32>().ok())
        else {
            return false;
        };

        mode_resolution == resolution && (rate - refresh_rate).abs() < 0.1
    })
}
