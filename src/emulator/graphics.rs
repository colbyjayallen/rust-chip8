use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};

const PIXEL_SIZE: u8 = 16;
const WINDOW_WIDTH: u8 = 64;
const WINDOW_HEIGHT: u8 = 32;

