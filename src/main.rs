struct Position {
    x: i8,
    y: i8,
}

type Movements = [(i8, i8); 1];

struct Knight {
    position: Position,
    movements: Movements,
}

impl Knight {
    fn make_move(&mut self) {
        let (x,y) = self.movements[0];
        self.position.x += x;
        self.position.y += y;
    }
}

fn main() {
    const BOARD_SIZE: usize = 10;
    let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
    board[2][1] = 17;
    // println!("{}", board[2][1]);
    let mut knight = Knight {
        position : Position {
            x:0,
            y:0,
        },
        movements : [(1,2)],
    };
    knight.make_move();
}
