use crate::io;
use crate::ls::Disp;

pub fn arange(disps: &[&Disp], c: i32) {
    let mut lx: i32 = 0;

    for disp in disps {
        io::position(disp, lx, 0);

        lx += (disp.width as f32 / disp.scale).round() as i32 + c;
    }
}
