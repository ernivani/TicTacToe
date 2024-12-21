use macroquad::prelude::*;

const CELL_SIZE: f32 = 100.0;
const BOARD_SIZE: f32 = CELL_SIZE * 3.0;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

#[macroquad::main("TicTacToe")]
async fn main() {
    let mut board = [[Cell::Empty; 3]; 3];
    let mut current_player = Cell::X;
    let mut game_over = false;
    let mut winner: Option<Cell> = None;

    loop {
        if is_mouse_button_pressed(MouseButton::Left) && !game_over {
            let (mouse_x, mouse_y) = mouse_position();
            let board_x = (screen_width() - BOARD_SIZE) / 2.0;
            let board_y = (screen_height() - BOARD_SIZE) / 2.0;

            // Convert mouse position to board coordinates
            if mouse_x >= board_x && mouse_x < board_x + BOARD_SIZE &&
               mouse_y >= board_y && mouse_y < board_y + BOARD_SIZE {
                let col = ((mouse_x - board_x) / CELL_SIZE) as usize;
                let row = ((mouse_y - board_y) / CELL_SIZE) as usize;
                
                if board[row][col] == Cell::Empty {
                    board[row][col] = current_player;
                    current_player = if current_player == Cell::X { Cell::O } else { Cell::X };
                    
                    // Check for winner
                    winner = check_winner(&board);
                    if winner.is_some() {
                        game_over = true;
                    } else if is_board_full(&board) {
                        game_over = true;
                    }
                }
            }
        }

        // Reset game if R is pressed
        if is_key_pressed(KeyCode::R) {
            board = [[Cell::Empty; 3]; 3];
            current_player = Cell::X;
            game_over = false;
            winner = None;
        }

        clear_background(WHITE);

        // Draw board
        let board_x = (screen_width() - BOARD_SIZE) / 2.0;
        let board_y = (screen_height() - BOARD_SIZE) / 2.0;

        // Draw grid lines
        for i in 0..=3 {
            let x = board_x + (i as f32 * CELL_SIZE);
            let y = board_y + (i as f32 * CELL_SIZE);
            draw_line(x, board_y, x, board_y + BOARD_SIZE, 2.0, BLACK);
            draw_line(board_x, y, board_x + BOARD_SIZE, y, 2.0, BLACK);
        }

        // Draw X's and O's
        for row in 0..3 {
            for col in 0..3 {
                let x = board_x + (col as f32 * CELL_SIZE);
                let y = board_y + (row as f32 * CELL_SIZE);
                match board[row][col] {
                    Cell::X => {
                        // Draw triangle for X
                        draw_triangle(
                            Vec2::new(x + CELL_SIZE/2.0, y + 20.0),
                            Vec2::new(x + 20.0, y + CELL_SIZE - 20.0),
                            Vec2::new(x + CELL_SIZE - 20.0, y + CELL_SIZE - 20.0),
                            BLUE
                        );
                    },
                    Cell::O => {
                        // Draw rectangle for O
                        draw_rectangle(
                            x + 20.0,
                            y + 20.0,
                            CELL_SIZE - 40.0,
                            CELL_SIZE - 40.0,
                            RED
                        );
                    },
                    Cell::Empty => {}
                }
            }
        }

        // Draw game status
        let status_text = if game_over {
            match winner {
                Some(Cell::X) => "X Wins! Press R to restart",
                Some(Cell::O) => "O Wins! Press R to restart",
                _ => "Draw! Press R to restart"
            }
        } else {
            match current_player {
                Cell::X => "X's turn",
                Cell::O => "O's turn",
                _ => ""
            }
        };
        
        let text_dims = measure_text(status_text, None, 30, 1.0);
        draw_text(
            status_text,
            (screen_width() - text_dims.width) / 2.0,
            50.0,
            30.0,
            BLACK
        );

        next_frame().await
    }
}

fn check_winner(board: &[[Cell; 3]; 3]) -> Option<Cell> {
    // Check rows
    for row in board {
        if row[0] != Cell::Empty && row[0] == row[1] && row[1] == row[2] {
            return Some(row[0]);
        }
    }

    // Check columns
    for col in 0..3 {
        if board[0][col] != Cell::Empty && 
           board[0][col] == board[1][col] && 
           board[1][col] == board[2][col] {
            return Some(board[0][col]);
        }
    }

    // Check diagonals
    if board[0][0] != Cell::Empty && 
       board[0][0] == board[1][1] && 
       board[1][1] == board[2][2] {
        return Some(board[0][0]);
    }
    
    if board[0][2] != Cell::Empty && 
       board[0][2] == board[1][1] && 
       board[1][1] == board[2][0] {
        return Some(board[0][2]);
    }

    None
}

fn is_board_full(board: &[[Cell; 3]; 3]) -> bool {
    board.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
}
