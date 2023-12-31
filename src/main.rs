use macroquad::prelude::*;

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
enum CellState {
    Alive,
    Dying,
    Dead,
}

#[macroquad::main("Life")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut cells = vec![CellState::Dead; w * h];
    let mut buffer = vec![CellState::Dead; w * h];

    // init state
    // for i in 0..cells.len() {
    //     if rand::gen_range(0, 3) == 0 {
    //         cells[i] = CellState::Alive;
    //     }
    // }

    // not sure wtf this does
    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    // main simulation loop
    loop {
        // break
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // update cell state
        for i in 0..cells.len() {
            // rule 30, alive if out of the cell and its neighbors, exactly 1 was alive

            let mut alive_count = 0;

            // check left
            if i > 0 && cells[i - 1] == CellState::Alive {
                alive_count += 1;
            }

            // curr
            if cells[i] == CellState::Alive {
                alive_count += 1;
            }

            // right
            if i < cells.len() - 1 && cells[i] == CellState::Alive {
                alive_count += 1;
            }

            // update cell state
            if alive_count == 1 {
                buffer[i] = CellState::Alive;
            } else if alive_count > 1 {
                buffer[i] = CellState::Dying;
            } else {
                buffer[i] = CellState::Dead;
            }
        }

        // spawn alive cells on mouse click
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let column = x as usize;
            let row = y as usize;
            let idx = row * w + column;
            buffer[idx] = CellState::Alive;
            println!("row: {}, col: {}, idx: {}", row, column, idx);
        }

        // render cell state
        for i in 0..cells.len() {
            cells[i] = buffer[i];
            image.set_pixel(
                (i % w) as u32,
                (i / w) as u32,
                match cells[i] {
                    CellState::Alive => BLACK,
                    CellState::Dying => ORANGE,
                    CellState::Dead => WHITE,
                },
            );
        }

        texture.update(&image);

        draw_texture(&texture, 0., 0., WHITE);

        next_frame().await
    }
}
