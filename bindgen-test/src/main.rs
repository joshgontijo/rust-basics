mod bindings;

use std::ffi::CString;
use bindings::{BeginDrawing, ClearBackground, CloseWindow, Color, DrawFPS, DrawRectangle, DrawText, EndDrawing, InitWindow, SetTargetFPS, WindowShouldClose};


fn main() {
    unsafe {
        InitWindow(800, 450, CString::new("Hello World").unwrap().as_ptr());

        SetTargetFPS(60);

        while !WindowShouldClose()
        {
            BeginDrawing();

            DrawFPS(10,10);

            ClearBackground(Color{
                r: 0,
                g: 0,
                b: 0,
                a: 0
            });


            DrawRectangle(50,50, 50, 50, Color{
                r: 255,
                g: 0,
                b: 0,
                a: 0
            });

            DrawText(CString::new("Congrats! You created your first window!").unwrap().as_ptr(), 100, 200, 20, Color{
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
