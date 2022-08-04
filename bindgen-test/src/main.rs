extern crate bindgen_test;

use std::ffi::CString;
use std::ptr::null;
use bindgen_test::{BeginDrawing, ClearBackground, CloseWindow, Color, DrawText, EndDrawing, InitWindow, WindowShouldClose};

fn main() {
    println!("Hello, world!");

    unsafe {
        InitWindow(800, 450, CString::new("Hello World").unwrap().as_ptr());

        while !WindowShouldClose()
        {
            BeginDrawing();
            ClearBackground(Color{
                r: 0,
                g: 0,
                b: 0,
                a: 0
            });
            DrawText(CString::new("Congrats! You created your first window!").unwrap().as_ptr(), 190, 200, 20, Color{
                r: 255,
                g: 0,
                b: 0,
                a: 0
            });
            EndDrawing();
        }

        CloseWindow();
    }
}
