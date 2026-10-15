mod shr;
mod ls;
mod io;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("list") => {
            let displ = ls::get_disp();

            for (i, disp) in displ.iter().enumerate() {
                println!(
                    "{}: {} {}x{} @ {:.2}Hz",
                    i + 1,
                    disp.name,
                    disp.width,
                    disp.height,
                    disp.refreshRate
                );
            }
        }

        Some("scale") | Some("refresh") => {
            let command = args[0].as_str();

            let Some(number) = args.get(1).and_then(|n| n.parse::<usize>().ok()) else {
                eprintln!("++=> distroller {command} <monitor number> <value>");
                return;
            };

            let Some(value) = args.get(2).and_then(|n| n.parse::<f32>().ok()) else {
                eprintln!("++=> distroller {command} <monitor number> <value>");
                return;
            };

            if command == "scale" && (!value.is_finite() || value <= 0.0 || value > 10.0) {
                eprintln!("scale is supposed to be bigger than 0 and smaller than 10.");
                return;
            }

            let displ = ls::get_disp();

            let Some(disp) = number.checked_sub(1).and_then(|i| displ.get(i)) else {
                eprintln!("moniter number out of range.");
                return;
            };

            if command == "refresh" && !io::sup_refresh(disp, value) {
                eprintln!(
                    "Unsupported refresh rate {value}Hz for {} at {}x{}.",
                    disp.name, disp.width, disp.height
                );
                return;
            }

            if command == "scale" {
                io::scale(disp, value);
            } else {
                io::refresh(disp, value);
            }

            println!("pplied `{command}` value `{value}` to {}", disp.name);
        }

        Some("enable") | Some("disable") => {
            let command = args[0].as_str();

            let Some(selection) = args.get(1) else {
                eprintln!("try +=> distroller {command} <monitor number(s)>");
                return;
            };

            let numbers: Result<Vec<usize>, _> =
                selection.split(',').map(str::parse).collect();

            let Ok(numbers) = numbers else {
                eprintln!("not valid monitor number(s).");
                return;
            };

            if numbers.is_empty() {
                eprintln!("select at least one monitor.");
                return;
            }

            if command == "disable" && numbers.len() != 1 {
                eprintln!("disable accepts only one monitor.");
                return;
            }

            let displ = ls::get_disp();

            let selected: Vec<_> = numbers
                .iter()
                .map(|number| {
                    number
                        .checked_sub(1)
                        .and_then(|i| displ.get(i))
                })
                .collect();

            if selected.iter().any(|disp| disp.is_none()) {
                eprintln!("One or more monitor numbers are out of range.");
                return;
            }

            for disp in selected.into_iter().flatten() {
                if command == "enable" {
                    io::enable(disp);
                } else {
                    io::disable(disp);
                }

                println!("{command} {}", disp.name);
            }
        }

        Some("use") => {
            let Some(selection) = args.get(1) else {
                eprintln!("++=> distroller use <monitor numbers> --gap <pixels>");
                return;
            };

            let numbers: Result<Vec<usize>, _> =
                selection.split(',').map(str::parse).collect();

            let Ok(numbers) = numbers else {
                eprintln!("not valid monitor number(s).");
                return;
            };

            let gap = if let Some(i) = args.iter().position(|arg| arg == "--gap") {
                let Some(value) = args.get(i + 1).and_then(|v| v.parse::<i32>().ok()) else {
                    eprintln!("++=> distroller use <monitor numbers> --gap <pixels>");
                    return;
                };
                value
            } else {
                10
            };

            let displ = ls::get_disp();

            let selected: Vec<_> = numbers
                .iter()
                .map(|number| number.checked_sub(1).and_then(|i| displ.get(i)))
                .collect();

            if selected.is_empty() || selected.iter().any(|disp| disp.is_none()) {
                eprintln!("One or more monitor numbers are out of range.");
                return;
            }

            let selected: Vec<_> = selected.into_iter().flatten().collect();

            for disp in &selected {
                io::enable(disp);
            }

            shr::arange(&selected, gap);
        }

        Some("help") | None => {
            println!("Distroller - Hyprland display manager");
            println!("Commands ->\n");
            println!(" => list");
            println!("   To show the current moniters");
            println!(" => scale <monitor number> <scale>");
            println!("   to scale the moniters");
            println!(" => refresh <monitor number> <hz>");
            println!("   to set specific refresh rate ");
            println!(" => enable <monitor numbers>");
            println!("   to enable one or more moniters");
            println!(" => disable <monitor number>");
            println!("   to disable one moniter");
        }

        Some(command) => eprintln!("Unknown command: {command}"),
    }
}
