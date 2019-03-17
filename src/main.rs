#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

type Movements = [(i32, i32); 8];

#[derive(Debug, Copy, Clone)]
struct Knight {
    position: Position,
    movements: Movements,
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    value: i32,
    is_visited: bool,
}

const BOARD_SIZE: usize = 100;

impl Knight {
    fn make_move(&mut self, board: &mut [[Cell; BOARD_SIZE]; BOARD_SIZE]) -> (Position, bool) {
        let mut index = std::usize::MAX;
        let mut value = std::i32::MAX;
        board[self.position.x as usize][self.position.y as usize].is_visited = true;
        for (i, movement) in self.movements.iter().enumerate() {
            let (x, y) = (movement.0, movement.1);
            if (self.position.x + x) < 0 ||
               (self.position.x + x) >= BOARD_SIZE as i32 ||
               (self.position.y + y) < 0 ||
               (self.position.y + y) >= BOARD_SIZE as i32 {
                continue;
            }
            if board[(self.position.x + x) as usize][(self.position.y + y) as usize].value < value && board[(self.position.x + x) as usize][(self.position.y + y) as usize].is_visited == false{
                value = board[(self.position.x + x) as usize][(self.position.y + y) as usize].value;
                index = i;
            }
        }
        if index == std::usize::MAX {
            return (self.position, false);
        }
        let (x, y) = self.movements[index];
        // print!("{} {} {} \n", index, self.position.x, self.position.y);
        print!("{0: <2} {1: <2} \n", self.position.x, self.position.y);
        // let (x_old_position, y_old_position) = (self.position.x, self.position.y);
        self.position.x += x;
        self.position.y += y;
        // print!("{} {} {} \n\n", index, self.position.x, self.position.y);
        // return (self.position.x, self.position.y);
        return (self.position, true);
        // plot.lines(&[x_old_position, self.position.x],
        //            &[y_old_position, self.position.y],
        //            &[Color("black")]);
    }
}

extern crate gnuplot;
use gnuplot::{Figure, Color, LineWidth , BorderColor, PointSize, PointSymbol};

fn main() {
    let mut counter = 1i32;
    // let mut board: Vec<Cell> = Vec::new();
    let mut board = [[Cell{value:0, is_visited:false}; BOARD_SIZE]; BOARD_SIZE];
    // for x in 0..BOARD_SIZE {
    for x in 0..BOARD_SIZE {
        for y in 0..x+1 {
            board[y][x-y].value = counter;
            // board[y][x-y].value = counter;
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

    let mut position_history: Vec<Position> = Vec::new();
    position_history.push(Position{x:0, y:0});

    let mut fg = Figure::new();
    // let axes2d = &mut fg.axes2d();
    for _i in 0..5000 {
        let (position, is_correct) = knight.make_move(&mut board);
        if !is_correct {
            break;
        }
        position_history.push(position);
    }
    let mut x_values: Vec<i32> = Vec::new();
    let mut y_values: Vec<i32> = Vec::new();
    for position in position_history {
        x_values.push(position.y);
        y_values.push(-position.x);
    }
    fg.axes2d().set_border(false, &[], &[LineWidth(0.0)])
               .lines(&x_values,
                      &y_values,
                      &[Color("blue"), BorderColor("white")])
               .points(x_values.last(),
                       y_values.last(),
                       &[PointSymbol('x'), Color("red"), PointSize(3.0)]);
    fg.show();
}
