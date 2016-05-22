extern crate sdl2;
use sdl2::keyboard::Scancode;

pub fn map_scancode(scancode: Scancode) -> Option<(i32, i32)> {
    use sdl2::keyboard::Scancode::*;
    match scancode {

        LShift => Some((-1,0)),
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
        RShift => Some((10,0)),

        CapsLock => Some((-2,1)),
        A => Some((-1,1)),
        S => Some((0,1)),
        D => Some((1,1)),
        F => Some((2,1)),
        G => Some((3,1)),
        H => Some((4,1)),
        J => Some((5,1)),
        K => Some((6,1)),
        L => Some((7,1)),
        Semicolon => Some((8,1)),
        Apostrophe => Some((9,1)),
        Return  => Some((10,1)),

        Tab => Some((-3,2)),
        Q => Some((-2,2)),
        W => Some((-1,2)),
        E => Some((0,2)),
        R => Some((1,2)),
        T => Some((2,2)),
        Y => Some((3,2)),
        U => Some((4,2)),
        I => Some((5,2)),
        O => Some((6,2)),
        P => Some((7,2)),
        LeftBracket => Some((8,2)),
        RightBracket => Some((9,2)),
        Backslash => Some((10,2)),

        Grave => Some((-4,3)),
        Num1 => Some((-3,3)),
        Num2 => Some((-2,3)),
        Num3 => Some((-1,3)),
        Num4 => Some((0,3)),
        Num5 => Some((1,3)),
        Num6 => Some((2,3)),
        Num7 => Some((3,3)),
        Num8 => Some((4,3)),
        Num9 => Some((5,3)),
        Num0 => Some((6,3)),
        Minus => Some((7,3)),
        Equals => Some((8,3)),
        Backspace => Some((9,3)),

        _ => None
    }
}

