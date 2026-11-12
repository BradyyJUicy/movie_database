/*
By: <Brady Big Canoe>
Date: 2026-11-12
Program Details: <A database program that allows users to add, edit, and delete movies from a database. The program will also allow users to search for movies by title, genre, or year. The program will be built using Rust and the Macroquad library for the GUI.>
*/

mod ui;
mod utils;

use crate::ui::grid::draw_grid;
use crate::ui::still_image::StillImage;

use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "movie_database".to_string(),
        window_width: 824,
        window_height: 968,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let img_bg = StillImage::new(
        "assets/mdbackground.png",
        824.0, // width
        968.0, // height
        0.0, // x position
        0.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    loop {
        clear_background(WHITE);


        img_bg.draw();
        draw_grid(50.0, BROWN);
        next_frame().await;
    }
}
