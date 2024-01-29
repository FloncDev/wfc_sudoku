pub mod sudoku;

use std::u32;

use macroquad::prelude::*;
use sudoku::{Sudoku, ValidationError};

const NUMBER_SIZE: f32 = 40.0;

// Not sure if this does anything but oh well
fn conf() -> Conf {
    Conf {
        window_title: String::from("Sudoku"),
        high_dpi: true,
        sample_count: 200,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut grid = Sudoku::new(3);

    let mut highlight_square: Option<(f32, f32)> = Some((1.0, 1.0));

    loop {
        clear_background(WHITE);
        let mouse_grid = get_grid(mouse_position());

        if is_mouse_button_pressed(MouseButton::Left) {
            highlight_square = None;
        } else if is_mouse_button_pressed(MouseButton::Right) {
            let coords = (mouse_grid.0 as u32, mouse_grid.1 as u32);
            let entropy = grid.get_entropy(coords).unwrap();

            if entropy.len() == 1 {
                grid.set(coords, entropy[0]).unwrap();
            }
        }

        if let Some(key) = get_last_key_pressed() {
            let coords = (mouse_grid.0 as u32, mouse_grid.1 as u32);
            let last_char = format!("{:#?}", key).chars().last().unwrap();
            if last_char.is_numeric() {
                match last_char.to_string().parse::<u32>().unwrap() {
                    0 => {
                        grid.unset(coords);
                    }
                    num @ _ => {
                        match grid.set(coords, num) {
                            Err(ValidationError::GroupHasSameNumber(x, y))
                            | Err(ValidationError::RegionHasSameNumber(x, y)) => {
                                highlight_square = Some((x as f32, y as f32));
                            }
                            Ok(_) => {
                                highlight_square = None;
                            }
                            _ => {}
                        };
                    }
                };
            }
        }

        if let Some((highlight_x, highlight_y)) = highlight_square {
            draw_rectangle(
                highlight_x * NUMBER_SIZE,
                highlight_y * NUMBER_SIZE,
                NUMBER_SIZE - 1.0,
                NUMBER_SIZE - 1.0,
                PINK,
            );
        }

        for x in 0..grid.n.pow(2) {
            for y in 0..grid.n.pow(2) {
                let (x, y) = (x as f32, y as f32);

                let region_x = (x / grid.n as f32).floor();
                let region_y = (y / grid.n as f32).floor();

                if let Some(num) = grid.get((x as u32, y as u32)) {
                    draw_text(
                        format!("{}", num).as_str(),
                        x * NUMBER_SIZE + (NUMBER_SIZE / 4.0),
                        (y + 1.0) * NUMBER_SIZE - (NUMBER_SIZE / 4.0),
                        NUMBER_SIZE,
                        BLACK,
                    );
                } else {
                    let entropy = grid.get_entropy((x as u32, y as u32));

                    draw_text(
                        format!("{}", entropy.unwrap().len()).as_str(),
                        x * NUMBER_SIZE + (NUMBER_SIZE / 4.0),
                        (y + 1.0) * NUMBER_SIZE - (NUMBER_SIZE / 4.0),
                        NUMBER_SIZE,
                        GREEN,
                    );
                    // draw_rectangle(
                    //     x * NUMBER_SIZE,
                    //     y * NUMBER_SIZE,
                    //     NUMBER_SIZE - 1.0,
                    //     NUMBER_SIZE - 1.0,
                    //     // Alternate colours
                    //     color_u8!(0, 255, 0, 255 * entropy),
                    // );
                }

                draw_rectangle(
                    x * NUMBER_SIZE,
                    y * NUMBER_SIZE,
                    NUMBER_SIZE - 1.0,
                    NUMBER_SIZE - 1.0,
                    // Alternate colours
                    if (region_x + region_y) % 2.0 == 0.0 {
                        color_u8!(0, 0, 0, 100)
                    } else {
                        color_u8!(0, 0, 0, 50)
                    },
                );
            }
        }

        next_frame().await
    }
}

fn get_grid(coords: (f32, f32)) -> (f32, f32) {
    (
        (coords.0 / NUMBER_SIZE).floor(),
        (coords.1 / NUMBER_SIZE).floor(),
    )
}
