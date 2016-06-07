use sdl2::pixels::Color;
use sdl2::pixels::Color::RGB;

pub fn background1() -> Color {
    RGB(200, 200, 200)
}

pub fn background2() -> Color {
    RGB(155, 155, 155)
}

pub fn active() -> Color {
    RGB(155, 55, 55)
}

pub fn inactive() -> Color {
    RGB(55, 55, 55)
}
