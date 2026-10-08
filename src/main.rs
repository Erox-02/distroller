mod ls;
mod io;

fn main() {
    let displ = ls::get_disp();
    io::disable(&displ[1]);
    io::enable(&displ[1]);

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