use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Write};
use std::process;

type Board = [[i8; 3]; 3];

struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq)]
enum Mark {
    O = 1,
    X = -1,
}

fn value_to_str(value: i8) -> &'static str {
    match value {
        1 => "o",
        -1 => "x",
        _ => " ",
    }
}

#[derive(PartialEq, Debug)]
struct Player {
    no: u8,
    mark: Mark,
}

const PLAYER1: Player = Player {
    no: 1,
    mark: Mark::O,
};

const PLAYER2: Player = Player {
    no: 2,
    mark: Mark::X,
};

fn init_board() -> Board {
    [[0; 3]; 3]
}

fn build_lines() -> Vec<Vec<Pos>> {
    let mut lines: Vec<Vec<Pos>> = vec![];

    let rows = (0..3)
        .map(|row| (0..3).map(|col| Pos { row, col }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cols = (0..3)
        .map(|col| (0..3).map(|row| Pos { row, col }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let diags = vec![
        (0..3).map(|x| Pos { row: x, col: x }).collect::<Vec<_>>(),
        (0..3)
            .map(|row| Pos { row, col: 2 - row })
            .collect::<Vec<_>>(),
    ];

    lines.extend(rows);
    lines.extend(cols);
    lines.extend(diags);

    lines
}

fn build_judge() -> impl Fn(&Board) -> Option<Player> {
    let lines = build_lines();

    move |board: &Board| -> Option<Player> {
        for line in lines.iter() {
            let sum: i8 = line.iter().map(|pos| board[pos.row][pos.col]).sum();
            if sum >= 3 {
                return Some(PLAYER1);
            }
            if sum <= -3 {
                return Some(PLAYER2);
            }
        }

        None
    }
}

fn board_str(board: &Board) -> String {
    board
        .iter()
        .map(|row| {
            let s = row
                .iter()
                .map(|v| format!("{:^3}", value_to_str(*v)))
                .collect::<Vec<_>>()
                .join("|");
            s + "\n"
        })
        .collect::<Vec<_>>()
        .join("-----------\n")
}

fn put(mark: Mark, board: &mut Board, pos: &Pos) {
    board[pos.row][pos.col] = mark as i8;
}

fn scan_idx(prompt: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-2]$").unwrap();
    }

    let mut buf = String::new();

    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    while !RE.is_match(buf.trim()) {
        println!("\nEnter 0, 1, or 2\n");
        buf.clear();

        print!("{prompt}");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
    }

    buf.trim().parse::<usize>().unwrap()
}

fn scan_pos(board: &Board) -> Pos {
    let mut row = scan_idx("row [0, 1, 2] > ");
    let mut col = scan_idx("col [0, 1, 2] > ");

    while board[row][col] != 0 {
        println!("\n({}, {}) is already taken.\n", row, col);
        row = scan_idx("row [0, 1, 2] > ");
        col = scan_idx("col [0, 1, 2] > ");
    }

    Pos { row, col }
}

pub fn game() {
    let mut board = init_board();
    let judge = build_judge();

    println!("{}", board_str(&board));

    for i in 0..(3 * 3) {
        let player = if i % 2 == 0 { PLAYER1 } else { PLAYER2 };
        println!("Player{}'s turn", player.no);

        let pos = scan_pos(&board);
        put(player.mark, &mut board, &pos);

        println!("\n{}", board_str(&board));

        match judge(&board) {
            Some(player) => {
                println!("Player{} won!", player.no);
                process::exit(0);
            }
            None => (),
        }
    }

    println!("Draw!")
}

#[cfg(test)]
mod judge_tests {
    use crate::{build_judge, PLAYER1, PLAYER2};

    #[test]
    fn judge_player1_won_by_row() {
        let board = [[-1, 0, 0], [1, 1, 1], [1, -1, -1]];
        let res = build_judge()(&board);
        assert_eq!(res, Some(PLAYER1));
    }

    #[test]
    fn judge_player1_won_by_col() {
        let board = [[-1, 1, 0], [1, 1, -1], [0, 1, -1]];
        let res = build_judge()(&board);
        assert_eq!(res, Some(PLAYER1));
    }

    #[test]
    fn judge_player1_won_by_topleft_to_bottomright() {
        let board = [[1, -1, 0], [1, 1, -1], [0, -1, 1]];
        let res = build_judge()(&board);
        assert_eq!(res, Some(PLAYER1));
    }

    #[test]
    fn judge_player1_won_by_topright_to_bottomleft() {
        let board = [[-1, 0, 1], [0, 1, -1], [1, -1, 1]];
        let res = build_judge()(&board);
        assert_eq!(res, Some(PLAYER1));
    }

    #[test]
    fn judge_player2_won_by_row() {
        let board = [[1, 0, 0], [-1, -1, -1], [-1, 1, 1]];
        let res = build_judge()(&board);
        assert_eq!(res, Some(PLAYER2));
    }

    #[test]
    fn judge_player2_won_by_col() {
        let board = [[1, -1, 0], [-1, -1, 1], [0, -1, 1]];
        let res = build_judge()(&board);
        assert_eq!(res, Some(PLAYER2));
    }

    #[test]
    fn judge_player2_won_by_topleft_to_bottomright() {
        let board = [[-1, 1, 0], [-1, -1, 1], [0, 1, -1]];
        let res = build_judge()(&board);
        assert_eq!(res, Some(PLAYER2));
    }

    #[test]
    fn judge_player2_won_by_topright_to_bottomleft() {
        let board = [[1, 0, -1], [0, -1, 1], [-1, 1, -1]];
        let res = build_judge()(&board);
        assert_eq!(res, Some(PLAYER2));
    }

    #[test]
    fn judge_neither_won() {
        let board = [[1, 0, -1], [0, 1, 1], [-1, 1, -1]];
        let res = build_judge()(&board);
        assert_eq!(res, None);
    }
}

#[cfg(test)]
mod board_str_tests {
    use crate::board_str;

    #[test]
    fn should_display_board_correctly() {
        let board = [[1, 0, -1], [0, -1, 1], [-1, 1, 0]];
        println!("{}", board_str(&board));
    }
}

#[cfg(test)]
mod scan_idx_tests {
    use crate::scan_idx;

    #[test]
    fn should_scan_valid_int() {
        scan_idx("row [0, 1, 2] > ");
    }
}

#[cfg(test)]
mod scan_pos_tests {
    use crate::{board_str, scan_pos};

    #[test]
    fn should_scan_valid_pos() {
        let board = [[1, 0, -1], [0, -1, 1], [-1, 1, 0]];
        println!("{}", board_str(&board));
        scan_pos(&board);
    }
}
