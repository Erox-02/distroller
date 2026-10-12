mod ls;
mod io;

fn main() {
    let displ = ls::get_disp();

    for disp in displ.iter() {
        println!(" display {}", disp.name);

        io::scale(disp, 1.33);
    }
}