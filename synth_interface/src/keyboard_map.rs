extern crate sdl2;
use sdl2::keyboard::Scancode;

pub fn map_scancode(scancode: Scancode) -> Option<(i32, i32)> {
    use sdl2::keyboard::Scancode::*;
    match scancode {
        Z => Some((0,0)),
        X => Some((1,0)),
        C => Some((2,0)),
        V => Some((3,0)),
        B => Some((4,0)),
        N => Some((5,0)),
        M => Some((6,0)),
        Comma => Some((7,0)),
        Period => Some((8,0)),
        Slash => Some((9,0)),

        /*
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),

        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),

        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        Z => Some((0,0)),
        */

        _ => None
    }
}

