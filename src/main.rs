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
        board[self.position.y as usize][self.position.x as usize].is_visited = true;
        for (i, movement) in self.movements.iter().enumerate() {
            let (y, x) = (self.position.y + movement.0, self.position.x + movement.1);
            if x < 0 || x >= BOARD_SIZE as i32 ||
               y < 0 || y >= BOARD_SIZE as i32 {
                continue;
            }
            let cell = board[y as usize][x as usize];
            if cell.value < value && cell.is_visited == false {
                value = cell.value;
                index = i;
            }
        }
        if index == std::usize::MAX {
            return (self.position, false);
        }
        // print!("{0: <2} {1: <2} \n", self.position.x, self.position.y);
        self.position.y += self.movements[index].0;
        self.position.x += self.movements[index].1;
        return (self.position, true);
    }
}

extern crate gnuplot;
use gnuplot::{Figure, Color, Auto, BorderColor, PointSize, PointSymbol, AxesCommon};

fn main() {
    let mut counter = 1i32;
    let mut board = [[Cell{value:0, is_visited:false}; BOARD_SIZE]; BOARD_SIZE];
    for x in 0..BOARD_SIZE {
        for y in 0..x+1 {
            board[y][x-y].value = counter;
            counter += 1;
        }
    }

    let mut knight = Knight {
        position : Position {x:0, y:0},
        movements : [(1,2),(2,1),(2,-1),(1,-2),(-1,-2),(-2,-1),(-2,1),(-1,2)],
    };

    let mut position_history: Vec<Position> = Vec::new();
    position_history.push(Position{x:0, y:0});

    for _i in 0..BOARD_SIZE*(BOARD_SIZE+1)/2 {
        let (position, is_correct) = knight.make_move(&mut board);
        if !is_correct {
            break;
        }
        position_history.push(position);
    }
    let mut x_values: Vec<i32> = Vec::new();
    let mut y_values: Vec<i32> = Vec::new();
    for position in position_history {
        x_values.push(position.x);
        y_values.push(position.y);
    }

    let mut fg = Figure::new();
    fg.axes2d().set_border(false, &[], &[])
               .set_y_reverse(true)
               .set_x_ticks(Some((Auto, 10-1)), &[], &[])
               .set_y_ticks(Some((Auto, 10-1)), &[], &[])
               .lines(&x_values,
                      &y_values,
                      &[Color("blue"), BorderColor("white")])
               .points(x_values.last(),
                       y_values.last(),
                       &[PointSymbol('O'), Color("red"), PointSize(1.0)]);
    fg.show();
}
