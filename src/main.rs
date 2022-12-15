use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

const NEUTRAL: char = '\0';

pub struct Board<const N: usize> {
    pub fields: [char; N],
}

impl<const N: usize> Board<N> {
    fn empty_cells(&self) -> usize {
        self.fields
            .iter()
            .map(|x| usize::from(x == &NEUTRAL))
            .sum()
    }
}

pub trait Player<const N: usize> {
    fn r#move(&mut self, board: &Board<N>) -> Board<N>;
}

pub struct RandomPlayer {
    pub symbol: char,
    pub rng: ThreadRng,
}

impl<const N: usize> Player<N> for RandomPlayer {
    fn r#move(&mut self, board: &Board<N>) -> Board<N> {
        let mut idx: usize = Uniform::from(0..board.fields.len()).sample(&mut self.rng);
        while board.fields[idx] != NEUTRAL {
            idx = (idx + 1) % board.fields.len();
        }
        let mut board_copy = Board {
            fields: board.fields,
        };
        board_copy.fields[idx] = self.symbol;
        board_copy
    }
}

pub struct FancyPlayer<const N: usize> {
    pub symbol: char,
    pub players: Vec<Box<dyn Player<N>>>,
}

impl<const N: usize> Player<N> for FancyPlayer<N> {
    fn r#move(&mut self, board: &Board<N>) -> Board<N> {
        Board {
            fields: board.fields,
        }
    }
}

pub trait Referee<const N: usize> {
    fn is_valid_move(&self, before: &Board<N>, after: &Board<N>) -> bool;
    fn r#final(&self, board: &Board<N>) -> Option<char>;
}

pub struct BasicReferee<const N: usize> {}

impl<const N: usize> Referee<N> for BasicReferee<N> {
    fn is_valid_move(&self, _before: &Board<N>, _after: &Board<N>) -> bool {
        true
    }

    fn r#final(&self, board: &Board<N>) -> Option<char> {
        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        for line in lines {
            let [a, b, c] = line;
            if board.fields[a] != NEUTRAL
                && board.fields[a] == board.fields[b]
                && board.fields[a] == board.fields[c]
            {
                return Some(board.fields[a]);
            }
        }
        if board.empty_cells() == 0 {
            return Some(NEUTRAL);
        }
        None
    }
}

fn main() {
    let mut board = Board { fields: [NEUTRAL; 9] };
    let referee: BasicReferee<9> = BasicReferee {};
    let mut player1 = RandomPlayer {
        symbol: 'X',
        rng: rand::thread_rng(),
    };
    let mut player2 = RandomPlayer {
        symbol: 'O',
        rng: rand::thread_rng(),
    };
    loop {
        board = player1.r#move(&board);
        println!("{:?}", board.fields);
        if let Some(winner) = referee.r#final(&board) {
            if winner == NEUTRAL {
                println!("Draw!");
            } else {
                println!("Winner: {}", winner);
            }
            break;
        }
        board = player2.r#move(&board);
        println!("{:?}", board.fields);
        if let Some(winner) = referee.r#final(&board) {
            if winner == NEUTRAL {
                println!("Draw!");
            } else {
                println!("Winner: {}", winner);
            }
            break;
        }
    }
}
