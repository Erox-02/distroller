mod ls;

fn main() {
    let displ = ls::get_disp();

    for disp in displ {
        println!(
            "{}: {} {}x{} @ {:.2}Hz",
            disp.id,
            disp.name,
            disp.width,
            disp.height,
            disp.refreshRate
        );
    }
}