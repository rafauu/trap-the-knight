struct Position {
    x: i32,
    y: i32,
}

type Movements = [(i32, i32); 8];

struct Knight {
    position: Position,
    movements: Movements,
}

// struct Board {
//     cells: Vec<Position>,
//     >
// }

const BOARD_SIZE: usize = 100;

impl Knight {
    fn make_move(&mut self, plot: &mut Axes2D, board: &[[i32; BOARD_SIZE]; BOARD_SIZE]) {
        let mut index = 0;
        let mut value = std::i32::MAX;
        for (i, movement) in self.movements.iter().enumerate() {
            let (x, y) = (movement.0, movement.1);
            if (self.position.x + x) < 0 ||
               (self.position.x + x) >= BOARD_SIZE as i32 ||
               (self.position.y + y) < 0 ||
               (self.position.y + y) >= BOARD_SIZE as i32 {
                continue;
            }
            if board[(self.position.x + x) as usize][(self.position.y + y) as usize] < value {
                value = board[(self.position.x + x) as usize][(self.position.y + y) as usize];
                index = i;
            }
        }
        let (x, y) = self.movements[index];
        print!("{} {} {} \n", index, self.position.x, self.position.y);
        let (x_old_position, y_old_position) = (self.position.x, self.position.y);
        self.position.x += x;
        self.position.y += y;
        print!("{} {} {} \n\n", index, self.position.x, self.position.y);
        // return (self.position.x, self.position.y);
        plot.lines(&[x_old_position, self.position.x],
                   &[y_old_position, self.position.y],
                   &[Color("black")]);
    }
}

extern crate gnuplot;
use gnuplot::{Figure, Color, Axes2D};

fn main() {
    let mut counter = 1i32;
    let mut board = [[0i32; BOARD_SIZE]; BOARD_SIZE];
    // for x in 0..BOARD_SIZE {
    for x in 0..BOARD_SIZE {
        for y in 0..x+1 {
            board[y][x-y] = counter;
            counter += 1;
        }
    }

    // for x in 0..BOARD_SIZE {
    //     for y in 0..BOARD_SIZE {
    //         print!("{0: <3}", board[x][y]);
    //     }
    //     print!("\n");
    // }

    let mut knight = Knight {
        position : Position {
            x:0,
            y:0,
        },
        movements : [(1,2),(2,1),(2,-1),(1,-2),(-1,-2),(-2,-1),(-2,1),(-1,2)],
    };

    let mut fg = Figure::new();
    knight.make_move(fg.axes2d(), &board);
    knight.make_move(fg.axes2d(), &board);
    knight.make_move(fg.axes2d(), &board);
    knight.make_move(fg.axes2d(), &board);
    fg.show();
}
